use sycamore::prelude::*;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt},
    MessageEvent, WebSocket,
};

fn connect(fp: &str, message: Signal<String>) -> WebSocket {
    let url = format!("{}?fp={}", crate::api::ws_url(), fp);
    let ws = WebSocket::new(&url).unwrap_throw();

    let cb = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(txt) = e.data().as_string() {
            message.set(txt);
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
    cb.forget();

    ws
}

#[component]
pub async fn Dashboard() -> View {
    let fp = crate::fingerprint::fingerprint();
    let message = create_signal(String::new());
    let ws = create_signal(connect(&fp, message));

    view! {
        p { "welcome to the arena" }
        (if message.get_clone().is_empty() {
            let fp2 = fp.clone();
            view! {
                p { "searching for opponent..." }
                button(on:click=move |_| {
                    ws.get_clone().close().ok();
                    ws.set(connect(&fp2, message));
                }) { "retry" }
            }
        } else {
            let fp3 = fp.clone();
            let result = message.get_clone();
            let label = if result == fp3 { "you won!" } else { "you lost." };
            view! {
                p { (label) }
            }
        })
    }
}
