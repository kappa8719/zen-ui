mod app;
mod router;
mod ui;

use crate::router::{Renderer, TransitionedHistoryIntegration};
use crate::ui::components::sidebar::Sidebar;
use js_sys::wasm_bindgen::JsCast;
use sycamore::prelude::*;
use sycamore::web::tags::*;
use sycamore_router::{Route, Router};

fn main() {
    sycamore::render(|| {
        view! {
            div(class = "w-dvw h-dvh fixed flex") {
                Sidebar()

                div(class = "flex-1 view-transition-navigate z-50") {
                    Router(
                        integration = TransitionedHistoryIntegration::new(),
                        view = |route: ReadSignal<router::Routes>| {
                            view! {
                                Renderer(route = route)
                            }
                        }
                    )
                }
            }
        }
    });
}
