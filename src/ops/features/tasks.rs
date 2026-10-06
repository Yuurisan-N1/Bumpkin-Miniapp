use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;
use crate::support::log::lm;

pub async fn run(eng: &mut Engine) -> Step {
    if !eng.cfg.enabled("tasks") {
        return Step::Idle("Task claiming is switched off in config.json".to_string());
    }

    if eng.send(reqs::task_list()).await.is_err() {
        return Step::Failed("The task list was not returned by the server for this account".to_string());
    }
    eng.settle(3000).await;

    if !eng.snap.tasks.seen {
        return Step::Idle("The task list was not returned by the server for this account".to_string());
    }

    let allow_social = eng.cfg.flag("tasks", "social");
    let allow_partner = eng.cfg.flag("tasks", "partner");
    let level = eng.snap.level();

    let mut claimed = 0usize;
    let mut opened = 0usize;

    let pending: Vec<_> = eng
        .snap
        .tasks
        .list
        .iter()
        .filter(|t| !t.done && t.level <= level)
        .map(|t| (t.id.clone(), t.section.clone(), t.left))
        .collect();

    for (id, section, left) in pending {
        let allowed = match section.as_str() {
            "social" => allow_social,
            "partner" => allow_partner,
            _ => true,
        };
        if !allowed || left > 0 {
            continue;
        }
        if !eng.cfg.flag("tasks", "claim") {
            continue;
        }
        if eng.send(reqs::task_claim(&id)).await.is_err() {
            return Step::Failed("A task claim was refused by the server on this run".to_string());
        }
        eng.settle(1400).await;
        claimed += 1;
    }

    let unopened: Vec<_> = eng
        .snap
        .tasks
        .list
        .iter()
        .filter(|t| !t.done && t.level <= level && t.left < 0)
        .map(|t| (t.id.clone(), t.section.clone()))
        .collect();

    for (id, section) in unopened {
        let allowed = match section.as_str() {
            "social" => allow_social,
            "partner" => allow_partner,
            _ => true,
        };
        if !allowed {
            continue;
        }
        if eng.send(reqs::task_open(&id)).await.is_err() {
            return Step::Failed("A task could not be opened on this run".to_string());
        }
        eng.settle(900).await;
        opened += 1;
    }

    if claimed == 0 && opened == 0 {
        let left = eng.snap.tasks.list.iter().filter(|t| !t.done).count();
        lm(&format!(
            "{} account tasks are still waiting and none were ready on this run",
            left
        ));
        return Step::Idle("Every available account task was already completed".to_string());
    }

    Step::Done(format!(
        "All available account tasks completed with {} rewards credited and {} tasks opened",
        claimed, opened
    ))
}
