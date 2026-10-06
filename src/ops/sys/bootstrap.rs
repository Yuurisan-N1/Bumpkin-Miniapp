use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::protocol::outbound::reqs;
use crate::support::log::{lm, ly};

pub async fn run(eng: &mut Engine) -> Step {
    let frames = [
        reqs::prices(),
        reqs::inventory(),
        reqs::task_list(),
        reqs::vip_state(),
        reqs::wheel_state(),
        reqs::bosses(),
        reqs::wb_info(),
    ];

    for frame in frames {
        if eng.send(frame).await.is_err() {
            return Step::Failed("The protocol state was not returned by the server on this run".to_string());
        }
    }

    eng.settle(3500).await;

    if !eng.alive() {
        return Step::Failed("The connection was lost during the protocol handshake on this run".to_string());
    }

    if eng.snap.name.is_empty() {
        lm("The welcome frame was not returned by the server on this run");
    }

    if eng.snap.uid > 0 && eng.acc.uid > 0 && eng.snap.uid != eng.acc.uid {
        ly("The server answered for a different account than the one in data.txt");
    }

    Step::Done(format!(
        "Protocol state was loaded for this account at level {} with {} boxes in the bag",
        eng.snap.level(),
        eng.snap.box_ids.len()
    ))
}
