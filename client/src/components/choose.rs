use sycamore::futures::spawn_local;
use sycamore::prelude::*;
use sycamore_router::navigate;
use web_sys::wasm_bindgen::UnwrapThrowExt;

use ebobo_shared::Fighter;

#[component]
pub async fn Fighters() -> View {
    let fighters = crate::api::available().await.unwrap_throw();
    let size = fighters.len();
    let available = create_signal(fighters);

    if size == 0 {
        view!(p { "empty here" })
    } else {
        view!( div {
            p { "choose your fighter:" }
            ul {
                Indexed(
                    list = *available,
                    view = move |f| view! {
                        SelectFighter(fighter = f.clone())
                    }
                )
            }
        })
    }
}

#[component(inline_props)]
pub async fn SelectFighter(fighter: Fighter) -> View {
    let name = fighter.0.clone();
    let selected: Signal<Option<Fighter>> = create_signal(None);

    create_effect(move || {
        if let Some(fighter) = selected.get_clone() {
            spawn_local(async move {
                crate::api::choose(fighter.0).await.unwrap_throw();
            });
        }
    });

    view! {
        li {
            button(on:click = move |_| {
                selected.set(Some(fighter.clone()));
                navigate("arena")
            }) { (name) }
        }
    }
}
