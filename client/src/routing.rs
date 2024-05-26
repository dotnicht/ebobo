use sycamore::prelude::*;
use sycamore_router::Route;

use crate::components;

#[derive(Route, Clone, Copy, Debug)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/choose")]
    Choose,
    #[not_found]
    NotFound,
}

#[component(inline_props)]
pub async fn Switch<G: Html>(route: ReadSignal<AppRoutes>) -> View<G> {
    view! {(match route.get() {
            AppRoutes::Index => view! { components::index::Index() },
            AppRoutes::Choose => view! { components::choose::Fighters() },
            AppRoutes::NotFound => view! { "404 Page Not Found"}
        })
    }
}