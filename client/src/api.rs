use reqwasm::http::Request;
use web_sys::wasm_bindgen::UnwrapThrowExt;

use ebobo_shared::*;

use crate::fingerprint::fingerprint;

pub async fn index() -> Result<Index, reqwasm::Error> {
    Ok(Request::get(&url())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .send()
        .await?
        .json()
        .await?)
}

pub async fn available() -> Result<Vec<Fighter>, reqwasm::Error> {
    Ok(Request::get(format!("{}/available", url()).as_str())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .send()
        .await?
        .json()
        .await?)
}

pub async fn choose(fighter: String) -> Result<(), reqwasm::Error> {
    Request::post(format!("{}/choose", url()).as_str())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .body(serde_json::to_string(&Choice(fighter)).unwrap_throw())
        .send()
        .await?;
    Ok(())
}

pub fn ws_url() -> String {
    let base = option_env!("EBOBO_API_URL")
        .unwrap_or("https://ebobo.shuttleapp.rs")
        .replace("https://", "wss://")
        .replace("http://", "ws://");
    format!("{}/fight", base)
}

fn url() -> String {
    option_env!("EBOBO_API_URL")
        .unwrap_or("https://ebobo.shuttleapp.rs")
        .to_owned()
}