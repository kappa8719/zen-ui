use crate::router;
use sycamore::prelude::*;

#[component]
pub fn Sidebar() -> View {
    view! {
        div(class = "w-64 h-full border-e-2 border-base-300 shadow-lg p-2 view-transition-disabled sidebar") {
            button(class = "btn btn-block justify-start transitions-all btn-soft", on:click = |_| router::navigate_with_transition("/")) {
                "Zen"
            }

            div(class="divider")

            div(class = "rounded-box bg-base-200 p-2") {
                button(class = "btn btn-outline btn-block btn-soft", on:click = |_| router::navigate_with_transition("/workspaces")) {
                    div(class = "icon-hexagon text-black")
                    "Workspaces"
                }
                div(class = "flex my-2 gap-2") {
                    div(class = "ms-4 border-x-1 border-base-300")
                    button(class = "btn flex-1") {
                        "Workspace 1"
                    }
                }
            }
        }
    }
}
