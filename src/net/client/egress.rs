use anyhow::{anyhow, Result};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct ProxyEntry {
    pub raw: String,
    pub kind: ProxyKind,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub pass: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyKind {
    Direct,
    Socks5,
    Http,
}

impl ProxyEntry {
    pub fn direct() -> Self {
        Self {
            raw: "direct".to_string(),
            kind: ProxyKind::Direct,
            host: String::new(),
            port: 0,
            user: None,
            pass: None,
        }
    }

    pub fn is_direct(&self) -> bool {
        self.kind == ProxyKind::Direct
    }

    pub fn label(&self) -> String {
        if self.is_direct() {
            "direct connection".to_string()
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }

    pub fn parse(line: &str) -> Result<Self> {
        let text = line.trim();
        if text.is_empty() || text.eq_ignore_ascii_case("direct") || text == "-" {
            return Ok(Self::direct());
        }

        let (scheme, rest) = match text.split_once("://") {
            Some((s, r)) => (s.to_ascii_lowercase(), r),
            None => ("socks5".to_string(), text),
        };

        let kind = match scheme.as_str() {
            "socks5" | "socks5h" | "socks" => ProxyKind::Socks5,
            "http" | "https" => ProxyKind::Http,
            other => return Err(anyhow!("unsupported proxy scheme {}", other)),
        };

        let (creds, hostport) = match rest.rsplit_once('@') {
            Some((c, h)) => (Some(c), h),
            None => (None, rest),
        };

        let (user, pass) = match creds {
            Some(c) => match c.split_once(':') {
                Some((u, p)) => (Some(u.to_string()), Some(p.to_string())),
                None => (Some(c.to_string()), None),
            },
            None => (None, None),
        };

        let (host, port) = hostport
            .rsplit_once(':')
            .ok_or_else(|| anyhow!("proxy line without port"))?;
        let port: u16 = port.parse().map_err(|_| anyhow!("bad proxy port"))?;
        if host.is_empty() {
            return Err(anyhow!("proxy line without host"));
        }

        Ok(Self {
            raw: text.to_string(),
            kind,
            host: host.trim_matches(['[', ']']).to_string(),
            port,
            user,
            pass,
        })
    }

    pub fn masked(&self) -> String {
        if self.is_direct() {
            return "direct connection".to_string();
        }
        format!("http://user:pass@{}:{}", mask_host(&self.host), self.port)
    }

    pub fn auth(&self) -> Option<(String, String)> {
        match (&self.user, &self.pass) {
            (Some(u), p) => Some((u.clone(), p.clone().unwrap_or_default())),
            _ => None,
        }
    }
}

fn mask_host(host: &str) -> String {
    let n = host.chars().count();
    let octets: Vec<&str> = host.split('.').collect();
    if octets.len() == 4 {
        return format!("{}*****{}", octets[0], octets[3]);
    }
    if n <= 4 {
        return "***".to_string();
    }
    let head: String = host.chars().take(2).collect();
    let tail: String = host.chars().skip(n - 2).collect();
    format!("{}*****{}", head, tail)
}

pub struct ProxyPool {
    entries: Vec<ProxyEntry>,
    cursor: usize,
}

impl ProxyPool {
    pub fn load_from(path: &str) -> Result<Self> {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let mut entries = Vec::new();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match ProxyEntry::parse(line) {
                Ok(e) => entries.push(e),
                Err(_) => continue,
            }
        }
        if entries.is_empty() {
            entries.push(ProxyEntry::direct());
        }
        Ok(Self { entries, cursor: 0 })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn all_direct(&self) -> bool {
        self.entries.iter().all(|e| e.is_direct())
    }

    pub fn next(&mut self) -> ProxyEntry {
        if self.entries.is_empty() {
            return ProxyEntry::direct();
        }
        let idx = self.cursor % self.entries.len();
        self.cursor = self.cursor.wrapping_add(1);
        self.entries[idx].clone()
    }

    pub fn pick(&self) -> ProxyEntry {
        if self.entries.is_empty() {
            return ProxyEntry::direct();
        }
        let idx = rand::thread_rng().gen_range(0..self.entries.len());
        self.entries[idx].clone()
    }

    pub fn masked(&self) -> String {
        match self.entries.first() {
            Some(e) => e.masked(),
            None => "direct connection".to_string(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProxyEntry> {
        self.entries.iter()
    }
}
