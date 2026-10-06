use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::support::log::lm;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("daily") {
        return Step::Idle("Daily missions are switched off in config.json".to_string());
    }

    if !eng.snap.daily.seen {
        eng.settle(2500).await;
    }

    if !eng.snap.daily.seen {
        return Step::Idle("The daily board was not returned by the server for this account".to_string());
    }

    let total = eng.snap.daily.list.len() as i64;
    let left = eng.snap.daily_left();

    if total > 0 && left == 0 {
        if eng.snap.daily.bonus {
            return Step::Done(format!(
                "Every daily mission was completed and the bonus was taken on streak day {}",
                eng.snap.daily.streak
            ));
        }
        return Step::Done(format!(
            "Every daily mission was already completed for this account on streak day {}",
            eng.snap.daily.streak
        ));
    }

    for mission in eng.snap.daily.list.clone() {
        if !mission.done {
            lm(&format!(
                "Daily mission {} is still pending with {} of {} done on this account",
                mission.title, mission.count, mission.need
            ));
        }
    }

    Step::Done(format!(
        "Daily missions {} of {} were completed on streak day {}",
        total - left,
        total,
        eng.snap.daily.streak
    ))
}
