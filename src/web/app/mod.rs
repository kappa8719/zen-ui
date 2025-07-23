use crate::ui::components::sidebar::Sidebar;
use sycamore::prelude::*;
use sycamore::{component, view};

pub mod workspaces;

#[component]
pub fn Page() -> View {
    view! {}
}

#[component]
pub fn Layout(children: Children) -> View {
    view! {
        div(class = "w-dvw h-dvh fixed flex") {
            Sidebar()

            div(class = "flex-1 z-100 bg-base-100") {
                (children)
            }
        }
    }
}
