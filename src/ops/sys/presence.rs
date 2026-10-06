use crate::ops::runner::Engine;
use crate::ops::Step;
use crate::support::clock::sleep_ms;
use crate::support::log::{lm, progress_end, progress_line};

pub async fn hold(eng: &mut Engine, seconds: u64) -> Step {
    let mut left = seconds;
    while left > 0 {
        if !eng.alive() {
            progress_end();
            return Step::Failed("The connection was lost while holding the session".to_string());
        }
        progress_line("Next cycle in", left);
        eng.drain(1000).await;
        left = left.saturating_sub(1);
    }
    progress_end();
    if !eng.alive() {
        return Step::Failed("The connection was lost while holding the session".to_string());
    }
    lm(&format!(
        "The connection was held for {} seconds with {} frames received on this account",
        seconds,
        eng.sock.frames()
    ));
    sleep_ms(10).await;
    Step::Done(format!(
        "The session was held for {} seconds on this account",
        seconds
    ))
}
