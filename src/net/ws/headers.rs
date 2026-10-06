use anyhow::Result;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::http::HeaderValue;

use crate::support::consts::{ORIGIN, REFERER, USER_AGENT};

pub fn handshake_request(url: &str) -> Result<Request> {
    let mut req = url.into_client_request()?;
    {
        let headers = req.headers_mut();
        headers.insert("Origin", HeaderValue::from_static(ORIGIN));
        headers.insert("Referer", HeaderValue::from_static(REFERER));
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("it-IT,it;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        headers.insert("Pragma", HeaderValue::from_static("no-cache"));
        headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    }
    Ok(req)
}

pub fn auth_frame(token: &str, lang: &str, dev: &str) -> String {
    let mut s = String::with_capacity(token.len() + dev.len() + 48);
    s.push_str("{\"t\":\"auth\",\"token\":\"");
    s.push_str(token);
    s.push_str("\",\"lang\":\"");
    s.push_str(lang);
    s.push_str("\",\"dev\":\"");
    s.push_str(dev);
    s.push_str("\"}");
    s
}
