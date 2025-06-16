mod app;
mod router;
mod ui;

use crate::router::{Renderer, TransitionedHistoryIntegration};
use crate::ui::components::Sidebar;
use js_sys::wasm_bindgen::JsCast;
use sycamore::prelude::*;
use sycamore::web::tags::*;
use sycamore_router::{Route, Router};

fn main() {
    sycamore::render(|| {
        view! {
            Router(
                integration = TransitionedHistoryIntegration::new(),
                view = |route: ReadSignal<router::Routes>| {
                    view! {
                        div(class = "w-dvw h-dvh flex") {
                            Sidebar()

                            div(class = "flex-1") {
                                Renderer(route = route)
                            }
                        }
                    }
                }
            )
        }
    });
}
