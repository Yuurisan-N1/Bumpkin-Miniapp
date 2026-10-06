use anyhow::{anyhow, Context, Result};
use serde_json::json;
use std::time::Duration;

use crate::net::client::egress::{ProxyEntry, ProxyKind};
use crate::support::consts::{API_TELEGRAM, BASE_URL};
use crate::support::log::{lm, sanitize, short};

pub struct Api {
    client: reqwest::Client,
    proxy: ProxyEntry,
    verbose: bool,
}

impl Api {
    pub fn new(proxy: ProxyEntry, verbose: bool) -> Result<Self> {
        let mut builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(25))
            .connect_timeout(Duration::from_secs(15))
            .user_agent(crate::support::consts::USER_AGENT);

        if !proxy.is_direct() {
            let scheme = match proxy.kind {
                ProxyKind::Socks5 => "socks5",
                _ => "http",
            };
            let url = match (&proxy.user, &proxy.pass) {
                (Some(u), Some(p)) => format!(
                    "{}://{}:{}@{}:{}",
                    scheme, u, p, proxy.host, proxy.port
                ),
                _ => format!("{}://{}:{}", scheme, proxy.host, proxy.port),
            };
            builder = builder.proxy(
                reqwest::Proxy::all(&url).with_context(|| format!("proxy {}", proxy.label()))?,
            );
        }

        Ok(Self {
            client: builder.build().context("http client")?,
            proxy,
            verbose,
        })
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn proxy_label(&self) -> String {
        self.proxy.label()
    }

    pub async fn login(&self, init_data: &str) -> Result<String> {
        let url = format!("{}{}", BASE_URL, API_TELEGRAM);
        let body = json!({ "initData": init_data });

        let resp = self
            .client
            .post(&url)
            .header("Origin", crate::support::consts::ORIGIN)
            .header("Referer", crate::support::consts::REFERER)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                anyhow!(
                    "the request to {} failed with {}",
                    API_TELEGRAM,
                    short(&sanitize(&e.to_string()), 70)
                )
            })?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if self.verbose {
            lm(&format!(
                "The server answered http {} with {}",
                status.as_u16(),
                short(&text, 90)
            ));
        }

        if !status.is_success() {
            return Err(anyhow!("http {}", status.as_u16()));
        }

        let v: serde_json::Value =
            serde_json::from_str(&text).with_context(|| format!("http body {}", short(&text, 60)))?;

        if let Some(err) = v.get("error").and_then(|x| x.as_str()) {
            return Err(anyhow!("the server refused the sign in with {}", sanitize(err)));
        }

        let token = v
            .get("token")
            .and_then(|x| x.as_str())
            .ok_or_else(|| anyhow!("the server answered without a session token"))?
            .to_string();

        if token.is_empty() {
            return Err(anyhow!("the server returned an empty session token"));
        }
        Ok(token)
    }
}
