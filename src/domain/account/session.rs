use anyhow::{anyhow, Result};

use crate::support::hash::{
    dev_id_for, has_hash, label_from_init_data, short_hash, user_id_from_init_data,
};

#[derive(Clone, Debug)]
pub struct Account {
    pub index: usize,
    pub init_data: String,
    pub uid: i64,
    pub name: String,
    pub dev: String,
    pub tag: String,
    pub token: Option<String>,
}

impl Account {
    pub fn parse(index: usize, init_data: &str) -> Result<Self> {
        let line = init_data.trim();
        if line.is_empty() {
            return Err(anyhow!("empty line"));
        }
        if !has_hash(line) {
            return Err(anyhow!("initData without hash"));
        }
        let uid = user_id_from_init_data(line).unwrap_or(0);
        let name = label_from_init_data(line);
        let tag = short_hash(line, 8);
        Ok(Self {
            index,
            init_data: line.to_string(),
            uid,
            name,
            dev: dev_id_for(line, "bl"),
            tag,
            token: None,
        })
    }

    pub fn label(&self) -> String {
        if self.name.is_empty() {
            format!("account {}", self.tag)
        } else {
            self.name.clone()
        }
    }

    pub fn dev_short(&self) -> String {
        self.dev.chars().take(8).collect()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

pub fn load_accounts(path: &str) -> Result<Vec<Account>> {
    let text = std::fs::read_to_string(path).map_err(|e| anyhow!("read {}: {}", path, e))?;
    let mut out = Vec::new();
    let mut skipped = 0usize;
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match Account::parse(out.len() + 1, line) {
            Ok(acc) => out.push(acc),
            Err(_) => skipped += 1,
        }
    }
    if out.is_empty() {
        return Err(anyhow!(
            "{} has no usable initData line ({} skipped)",
            path,
            skipped
        ));
    }
    Ok(out)
}
