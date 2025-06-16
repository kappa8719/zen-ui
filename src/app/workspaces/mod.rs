use sycamore::prelude::*;

#[component]
pub fn Page() -> View {
    view! {
        div(class = "p-8 size-full") {
            div(class = "font-semibold text-3xl") {
                "Workspaces"
            }

            div(class = "stats shadow") {
                div(class = "stat") {
                    // div(class = "stat-title") { "" }
                    div(class = "stat-value") { "Create new workspace" }
                    div(class = "stat-description") { "A"}
                }
            }
        }
    }
}
