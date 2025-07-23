use sycamore::prelude::*;
use crate::ui::workspaces::CreateWorkspaceDialog;

#[component]
pub fn Page() -> View {
    let is_create_workspace_dialog_open = create_signal(false);

    view! {
        CreateWorkspaceDialog(is_open = is_create_workspace_dialog_open)

        div(class = "p-8 size-full flex flex-col gap-4") {
            div(class = "flex") {
                div(class = "font-semibold text-3xl") { "Workspaces" }
                button(class = "btn btn-primary ml-4", on:click = move |_| is_create_workspace_dialog_open.set(true)) {
                    div(class = "icon-plus")
                    "Create Workspace"
                }
            }

            div(class = "overflow-x-auto rounded-box border border-base-content/5 bg-base-100") {
                table(class = "table") {
                    thead {
                        tr {
                            th()
                            th { "Name" }
                            th { "Name" }
                            th { "Name" }
                        }
                    }
                }
            }
        }
    }
}
