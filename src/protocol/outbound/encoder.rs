use anyhow::Result;
use serde_json::Value;

use crate::net::ws::session::Socket;
use crate::support::clock::now_ms;
use crate::support::consts::MAX_REPLY_BYTES;
use crate::support::log::{lm, sanitize, short};

pub fn encode(v: &Value) -> String {
    v.to_string()
}

pub struct Sender {
    min_gap_ms: u64,
    last_ms: i64,
    sent: u64,
    verbose: bool,
}

impl Sender {
    pub fn new(min_gap_ms: u64, verbose: bool) -> Self {
        Self {
            min_gap_ms,
            last_ms: 0,
            sent: 0,
            verbose,
        }
    }

    pub fn sent(&self) -> u64 {
        self.sent
    }

    pub async fn send(&mut self, sock: &mut Socket, frame: &Value) -> Result<()> {
        let gap = self.min_gap_ms as i64;
        let wait = gap - (now_ms() - self.last_ms);
        if wait > 0 {
            crate::support::clock::sleep_ms(wait as u64).await;
        }
        let payload = encode(frame);
        let clipped = if payload.len() > MAX_REPLY_BYTES {
            payload[..MAX_REPLY_BYTES].to_string()
        } else {
            payload.clone()
        };
        if self.verbose {
            lm(&format!("send {}", sanitize(&short(&clipped, 110))));
        }
        sock.send(&clipped).await?;
        self.last_ms = now_ms();
        self.sent += 1;
        Ok(())
    }
}
