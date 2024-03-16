use crate::{fingerprint, url};
use ebobo_shared::{Auth, Fighter};
use reqwasm::http::Request;
use sycamore::prelude::*;
use web_sys::RequestMode;

#[component(inline_props)]
pub async fn Auth<G: Html>() -> View<G> {
    let addr = Request::get("https://api.ipify.org")
        .mode(RequestMode::Cors)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let fighter = post(&url(), &fingerprint(), &addr).await.unwrap();

    let greet = match fighter.fighter {
        Some(f) => format!("Welcome back, {}!", f),
        None => format!("Welcome, {}!", fighter.fingerprint),
    };

    view! {
        p {
            (greet)
        }
    }
}

async fn post(url: &str, fingerprint: &str, host: &str) -> Result<Fighter, reqwasm::Error> {
    Ok(Request::post(format!("{}/authenticate", url).as_str())
        .header("EBOBO_FINGERPRINT", fingerprint)
        .body(
            serde_json::to_string(&Auth {
                fingerprint: fingerprint.to_string(),
                addr: host.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await?
        .json()
        .await?)
}
