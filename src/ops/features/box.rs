use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;
use crate::support::log::lg;

const MAX_BOXES_PER_CYCLE: usize = 6;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("box") {
        return Step::Idle("Daily box opening is switched off in config.json".to_string());
    }

    if eng.send(reqs::inventory()).await.is_err() {
        return Step::Failed("The inventory was not returned by the server on this run".to_string());
    }
    eng.settle(2500).await;

    let boxes = eng.snap.open_boxes();
    if boxes.is_empty() {
        return Step::Idle("Every reward box was already opened for this account".to_string());
    }

    let limit = if eng.cfg.flag("box", "open_all") {
        MAX_BOXES_PER_CYCLE
    } else {
        1
    };

    let mut opened = 0usize;
    for id in boxes.into_iter().take(limit) {
        if eng.send(reqs::box_open(&id)).await.is_err() {
            return Step::Failed("Gift box could not be opened on this run".to_string());
        }
        eng.settle(2600).await;
        opened += 1;
        if !eng.alive() {
            return Step::Failed("The connection was lost while opening boxes on this run".to_string());
        }
    }

    if eng.send(reqs::inventory()).await.is_err() {
        return Step::Failed("The inventory could not be refreshed on this run".to_string());
    }
    eng.settle(1800).await;

    lg(&format!(
        "Gift boxes {} were opened on this account with {} boxes left in the bag",
        opened,
        eng.snap.box_ids.len()
    ));
    Step::Done(format!(
        "Gift box opening finished with {} boxes opened on this account",
        opened
    ))
}
