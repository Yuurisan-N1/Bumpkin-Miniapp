use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("vip") {
        return Step::Idle("The VIP claim is switched off in config.json".to_string());
    }

    if eng.send(reqs::vip_state()).await.is_err() {
        return Step::Failed("The VIP state was not returned by the server on this run".to_string());
    }
    eng.settle(2000).await;

    if !eng.snap.vip.seen {
        return Step::Idle("The VIP state was not returned by the server on this run".to_string());
    }

    if !eng.snap.vip.daily {
        return Step::Idle(format!(
            "The VIP play bonus was already claimed on this account at level {}",
            eng.snap.vip.level
        ));
    }

    if eng.snap.vip.level <= 0 {
        return Step::Idle("The VIP play bonus is not unlocked yet on this account".to_string());
    }

    if !eng.cfg.flag("vip", "claim") {
        return Step::Idle("The VIP claim is switched off in config.json".to_string());
    }

    if eng.send(reqs::vip_claim()).await.is_err() {
        return Step::Failed("The VIP play bonus was refused by the server on this run".to_string());
    }
    eng.settle(2500).await;
    Step::Done(format!(
        "VIP play bonus was claimed on this account at level {}",
        eng.snap.vip.level
    ))
}
