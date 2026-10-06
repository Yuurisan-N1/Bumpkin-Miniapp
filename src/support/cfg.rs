use anyhow::{Context, Result};
use serde_json::Value;

#[derive(Clone)]
pub struct Config {
    raw: Value,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| format!("read {}", path))?;
        let raw: Value = serde_json::from_str(&text).with_context(|| format!("parse {}", path))?;
        Ok(Self { raw })
    }

    fn node(&self, section: &str) -> Option<&Value> {
        self.raw.get(section)
    }

    pub fn enabled(&self, section: &str) -> bool {
        match self.node(section) {
            Some(v) => v.get("enabled").and_then(|x| x.as_bool()).unwrap_or(false),
            None => false,
        }
    }

    pub fn flag(&self, section: &str, key: &str) -> bool {
        self.node(section)
            .and_then(|v| v.get(key))
            .and_then(|x| x.as_bool())
            .unwrap_or(false)
    }

    pub fn num(&self, section: &str, key: &str) -> i64 {
        self.node(section)
            .and_then(|v| v.get(key))
            .and_then(|x| x.as_i64())
            .unwrap_or(0)
    }

    pub fn text(&self, section: &str, key: &str) -> String {
        self.node(section)
            .and_then(|v| v.get(key))
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string()
    }

    pub fn loop_enabled(&self) -> bool {
        self.enabled("loop_cycle")
    }

    pub fn sleep_seconds(&self) -> i64 {
        self.num("loop_cycle", "sleep_seconds").max(60)
    }

    pub fn state_sync_seconds(&self) -> i64 {
        self.num("loop_cycle", "state_sync_seconds").max(30)
    }

    pub fn account_gap_seconds(&self) -> i64 {
        self.num("loop_cycle", "account_gap_seconds").max(5)
    }

    pub fn max_cycles(&self) -> i64 {
        self.num("loop_cycle", "max_cycles")
    }

    pub fn language(&self) -> String {
        let lang = self.text("profile", "language");
        if lang.is_empty() {
            "it".to_string()
        } else {
            lang
        }
    }

    pub fn hold_seconds(&self) -> u64 {
        self.num("loop_cycle", "hold_seconds").max(30) as u64
    }

    pub fn verify_max_tries(&self) -> i64 {
        self.num("gate", "verify_max_tries").max(1)
    }

    pub fn verify_retry_seconds(&self) -> i64 {
        self.num("gate", "verify_retry_seconds").max(5)
    }

    pub fn raw(&self) -> &Value {
        &self.raw
    }
}
