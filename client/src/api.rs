use reqwasm::http::Request;
use serde::Deserialize;
use wasm_fingerprint::make_fingerprint;

use ebobo_shared::Fighter;

#[derive(Deserialize)]
struct Fingerprint {
    print: String,
}

fn fingerprint() -> String {
    let fingerprint: Fingerprint =
        serde_json::from_str(&make_fingerprint().expect("Fingerprint not available"))
            .expect("Failed to deserialize fingerprint");
    fingerprint.print
}

fn url() -> String {
    option_env!("EBOBO_API_URL")
        .unwrap_or("https://ebobo.shuttleapp.rs")
        .to_owned()
}

pub async fn get() -> Result<String, reqwasm::Error> {
    Ok(Request::get(&url())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .send()
        .await?
        .text()
        .await?)
}

pub async fn choose(fighter: &str) -> Result<(), reqwasm::Error> {
    match Request::post(format!("{}/choose", url()).as_str())
        .header(ebobo_shared::AUTH_HEADER, &fingerprint())
        .body(fighter.to_owned())
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
