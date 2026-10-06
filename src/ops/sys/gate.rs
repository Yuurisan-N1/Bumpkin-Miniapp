use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;
use crate::support::log::{lg, ly};

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("gate") {
        return Step::Idle("The channel gate is switched off in config.json".to_string());
    }

    if eng.snap.gate.ok {
        return Step::Done("Channel gate is already joined on this account".to_string());
    }

    let tries = eng.cfg.verify_max_tries();
    let gap = eng.cfg.verify_retry_seconds() as u64;

    for attempt in 1..=tries {
        if eng.send(reqs::verify()).await.is_err() {
            return Step::Failed("The channel gate could not be checked for this account".to_string());
        }
        eng.settle(2500).await;

        if eng.snap.gate.ok {
            return Step::Done(format!(
                "Channel gate was cleared on attempt {} for this account",
                attempt
            ));
        }

        if !eng.snap.gate.channel {
            ly("Channel join is missing for this account so mining stays locked");
        }
        if !eng.snap.gate.group {
            ly("Community membership is missing for this account so mining stays locked");
        }
        if !eng.snap.gate.story {
            ly("The story share is missing for this account so the gate stays locked");
        }
        if !eng.snap.gate.bio {
            ly("The invite link is missing from the Telegram bio on this account");
        }
        if !eng.snap.gate.nick {
            ly("The project name is missing from the Telegram name on this account");
        }
        if let Some(reason) = eng.snap.gate.reason.clone() {
            ly(&format!("The gate was refused by the server with {}", reason));
        }

        if attempt < tries {
            crate::support::log::countdown(gap, "Retry in").await;
        }
    }

    for link in eng.snap.gate.links.clone() {
        lg(&format!("The gate link that must be opened for this account is {}", link));
    }
    Step::Failed("The channel gate was not cleared for this account on this run".to_string())
}
