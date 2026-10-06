use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;
use crate::support::log::{lg, ly};

const LEVEL_SLACK: i64 = 3;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("boss") {
        return Step::Idle("The boss phase is switched off in config.json".to_string());
    }

    if eng.cfg.flag("boss", "world_boss") {
        if eng.send(reqs::wb_info()).await.is_err() {
            return Step::Failed("The world boss state was not returned by the server".to_string());
        }
        eng.settle(2000).await;

        let wb = eng.snap.world_boss.clone();
        if wb.active && !wb.joined {
            if eng.send(reqs::wb_join()).await.is_err() {
                return Step::Failed("The world boss join was refused by the server".to_string());
            }
            eng.settle(3000).await;
            lg(&format!(
                "World boss {} was joined on this account",
                wb.name
            ));
        }
    }

    if !eng.cfg.flag("boss", "enter") {
        return Step::Idle("Every battle allowed for today was played on this account".to_string());
    }

    if eng.send(reqs::bosses()).await.is_err() {
        return Step::Failed("The boss list was not returned by the server on this run".to_string());
    }
    eng.settle(2500).await;

    if !eng.snap.bosses.seen {
        return Step::Idle("The boss list was not returned by the server on this run".to_string());
    }

    let level = eng.snap.level();
    let max_per_day = eng.snap.bosses.max_per_day.max(1);

    let pick = eng
        .snap
        .bosses
        .list
        .iter()
        .find(|b| b.cooldown <= 0 && b.today < max_per_day && level + LEVEL_SLACK >= b.level)
        .map(|b| (b.id.clone(), b.name.clone(), b.level));

    let (id, name, blevel) = match pick {
        Some(p) => p,
        None => {
            return Step::Idle(
                "Every battle allowed for today was played on this account".to_string(),
            );
        }
    };

    if eng.snap.in_boss_room {
        if eng.send(reqs::boss_leave()).await.is_err() {
            return Step::Failed("The boss room could not be left on this run".to_string());
        }
        eng.settle(1500).await;
    }

    if eng.send(reqs::boss_enter(&id)).await.is_err() {
        return Step::Failed("The boss entry was refused by the server on this run".to_string());
    }
    lg(&format!(
        "Boss {} at level {} was entered on this account",
        name, blevel
    ));
    eng.settle(6000).await;

    if !eng.alive() {
        return Step::Failed("The connection was lost during the boss fight on this run".to_string());
    }

    if eng.snap.in_boss_room {
        ly(&format!(
            "The boss fight against {} was left open past the window on this account",
            name
        ));
        return Step::Done(format!("Boss {} was fought on this account", name));
    }

    Step::Done(format!("Boss {} window was completed on this account", name))
}
