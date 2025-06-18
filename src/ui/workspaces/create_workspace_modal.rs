use sycamore::prelude::*;
use sycamore::prelude::{Attributes, View};
use sycamore::web::tags::*;
use sycamore::{component, view};
use sycamore::rt::console_error;
use web_sys::HtmlDialogElement;
use web_sys::wasm_bindgen::JsValue;

#[component(inline_props)]
pub fn CreateWorkspaceModal(#[prop(attributes(html, dialog))] attributes: Attributes, is_open: Signal<bool>) -> View {
    let dialog_ref = create_node_ref();

    create_effect(move || {
        let Some(node) = dialog_ref.try_get() else {
            console_error!("no node found");
            return;
        };

        let v: &JsValue = node.as_ref();
        let e = HtmlDialogElement::from(v.clone());

        if is_open.get() {
            console_log!("opening");
            e.show_modal();
        } else {
            is_open.set(false);
            e.close();
        }
    });

    view! {
        dialog(..attributes, class = "modal", r#ref=dialog_ref, on:close = move |_| { is_open.set(false) }) {
            div(class = "modal-box") {
                h3(class = "text-lg font-bold") { "Hello" }
            }
            form(method = "dialog", class = "modal-backdrop") {
                button { "close" }
            }
        }
    }
}
