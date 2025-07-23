use crate::interop::dom::{FromFn, NodeRefAsHtmlElementExt};
use js_sys::Function;
use js_sys::wasm_bindgen::JsCast;
use js_sys::wasm_bindgen::closure::Closure;
use sycamore::prelude::*;
use sycamore::prelude::{Attributes, View};
use sycamore::rt::console_error;
use sycamore::web::Portal;
use sycamore::web::tags::*;
use sycamore::{component, view};
use web_sys::wasm_bindgen::JsValue;
use web_sys::{HtmlDialogElement, HtmlElement};

#[component(inline_props)]
pub fn CreateWorkspaceDialog(
    #[prop(attributes(html, dialog))] attributes: Attributes,
    is_open: Signal<bool>,
) -> View {
    let dialog_ref = create_node_ref();
    let workspace_name_input_ref = create_node_ref();
    let workspace_name_input_value = create_signal(String::new());

    create_effect(move || {
        let Some(node) = dialog_ref.try_get() else {
            return;
        };

        let v: &JsValue = node.as_ref();
        let e = HtmlDialogElement::from(v.clone());

        if is_open.get() {
            let _ = e.show_modal();
            let c = Closure::from_fn(move || {
                console_log!("focusing");
                workspace_name_input_ref
                    .as_html_element::<HtmlElement>()
                    .focus();
            });
            let id = window()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Function::from(c.into_js_value()),
                    50,
                )
                .unwrap();
        } else {
            is_open.set(false);
            e.close();
        }
    });

    view! {
        Portal(selector = ":root") {
            dialog(..attributes, class = "modal ", r#ref=dialog_ref, on:close = move |_| { is_open.set(false) }) {
                div(class = "modal-box flex flex-col gap-1") {
                    h3(class = "text-lg font-bold") { "Create new workspace" }
                    fieldset(class = "fieldset") {
                        legend(class = "fieldset-legend") { "Workspace name" }
                        input(r#type = "text", r#ref = workspace_name_input_ref, bind:value = workspace_name_input_value, required = true, minlength = "5", maxlength = "63", class = "input validator w-full")
                        p(class = "label") { (format!("{}/63", workspace_name_input_value.get_clone().len())) }
                        p(class = "validator-hint") { "Length must be in range of 1-63" }
                    }
                    div(class = "w-full flex") {
                        button(class = "btn btn-primary ml-auto") {
                            "Create"
                        }
                    }
                }
                form(method = "dialog", class = "modal-backdrop") {
                    button { "close" }
                }
            }
        }
    }
}
