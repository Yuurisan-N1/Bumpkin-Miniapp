use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::inbound::decoder::Event;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("quest") {
        return Step::Idle("Quests are switched off in config.json".to_string());
    }

    let events = eng.drain(5000).await;
    let done = events
        .iter()
        .filter(|e| matches!(e, Event::QuestDone(_)))
        .count();

    if done > 0 {
        return Step::Done(format!(
            "{} quests were completed by the server on this account",
            done
        ));
    }

    Step::Idle("No quest was completed for this account on this run".to_string())
}
