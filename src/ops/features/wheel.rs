use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("wheel") {
        return Step::Idle("Spin wheel is switched off in config.json".to_string());
    }

    if eng.send(reqs::wheel_state()).await.is_err() {
        return Step::Failed("The wheel state was not returned by the server on this run".to_string());
    }
    eng.settle(2000).await;

    if !eng.snap.wheel.seen {
        return Step::Idle("The wheel state was not returned by the server on this run".to_string());
    }

    if eng.snap.wheel.free && eng.cfg.flag("wheel", "free") {
        if eng.send(reqs::wheel_free()).await.is_err() {
            return Step::Failed("The free wheel spin was refused by the server".to_string());
        }
        eng.settle(4000).await;
        return Step::Done(format!(
            "Free wheel spin was used on this account with {} spins left",
            eng.snap.wheel.spins
        ));
    }

    if eng.snap.vip.wheel_left > 0 && eng.cfg.flag("wheel", "vip") {
        if eng.send(reqs::wheel_vip()).await.is_err() {
            return Step::Failed("The VIP wheel spin was refused by the server".to_string());
        }
        eng.settle(4000).await;
        return Step::Done(format!(
            "VIP wheel spin was used on this account with {} spins left",
            eng.snap.vip.wheel_left
        ));
    }

    Step::Idle("Every free wheel spin was already used for this account".to_string())
}
