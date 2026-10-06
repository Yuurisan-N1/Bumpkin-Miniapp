use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub fn now_secs() -> i64 {
    now_ms() / 1000
}

pub fn hms(seconds: i64) -> String {
    let s = seconds.max(0);
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

pub fn wait_ms(deadline_ms: i64) -> i64 {
    (deadline_ms - now_ms()).max(0)
}

pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}

pub async fn sleep_secs(secs: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
}

pub struct Clock {
    start: i64,
}

impl Clock {
    pub fn new() -> Self {
        Self { start: now_ms() }
    }

    pub fn elapsed_ms(&self) -> i64 {
        now_ms() - self.start
    }

    pub fn elapsed(&self) -> String {
        let secs = self.elapsed_ms() / 1000;
        format!("{}m {}s", secs / 60, secs % 60)
    }

    pub fn tick(&self) -> String {
        format!("[{}]", hms(self.elapsed_ms() / 1000))
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}
