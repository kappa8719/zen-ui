mod interop;
mod router;
mod ui;
mod web;

use crate::router::{Renderer, ShouldShowSidebar, TransitionedHistoryIntegration};
use js_sys::wasm_bindgen::JsCast;
use sycamore::prelude::*;
use sycamore::web::tags::*;
use sycamore_router::{Route, Router};

fn main() {
    router::register_hooks();

    sycamore::render(|| {
        view! {
            Router(
                integration = TransitionedHistoryIntegration::new(),
                view = |route: ReadSignal<router::Routes>| {
                    view! {
                        Renderer(route = route)
                    }
                }
            )
        }
    });
}
