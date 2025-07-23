use crate::router;
use crate::ui::workspaces::CreateWorkspaceDialog;
use sycamore::prelude::*;

#[component]
pub fn Sidebar() -> View {
    let is_create_workspace_modal_open = create_signal(false);

    view! {
        div(class = "w-64 min-w-64")
        div(class = "w-64 h-full border-e-2 border-base-300 shadow-lg p-2 fixed view-transition-fixed z-[101] sidebar bg-base-100", data-transition = "true") {
            button(class = "btn btn-block justify-start transitions-all btn-soft", on:click = |_| router::navigate_with_transition("/")) {
                "Zen"
            }

            div(class="divider")

            div(class = "rounded-box bg-base-200 p-2") {
                div(class = "join flex") {
                    button(class = "join-item btn btn-soft flex-1", on:click = |_| router::navigate_with_transition("/workspaces")) {
                        div(class = "icon-hexagon text-black")
                        "Workspaces"
                    }
                    button(class = "join-item btn btn-ghost", on:click = move |_| { is_create_workspace_modal_open.update(|v| {*v = !*v})}) {
                        div(class = "icon-circle-plus")
                    }
                }
                div(class = "flex my-2 gap-2") {
                    div(class = "ms-4 border-x-1 border-base-300")
                    button(class = "btn flex-1") {
                        "Workspace 1"
                    }
                }
            }

            button(class = "btn flex-1 btn-primary btn-block mt-2", on:click = |_| router::navigate_with_transition("/auth")) {
                "Auth"
            }

            CreateWorkspaceDialog(is_open = is_create_workspace_modal_open)
        }
    }
}
