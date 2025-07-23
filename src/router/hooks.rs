use crate::router::{navigate, navigate_with_transition};
use js_sys::wasm_bindgen::closure::Closure;
use js_sys::wasm_bindgen::JsValue;
use sycamore::prelude::document;
use web_sys::{EventTarget, HtmlAnchorElement, HtmlElement, MouseEvent};
use web_sys::wasm_bindgen::JsCast;

pub fn register_hooks() {
    let f = Closure::wrap(Box::new(move |e: MouseEvent| {
        if let Some(target) = e.target() {
            let element = HtmlElement::from(<EventTarget as AsRef<JsValue>>::as_ref(&target).clone());
            if element.matches("a:not([data-no-transition])").unwrap_or(false) {
                let element = HtmlAnchorElement::from(<EventTarget as AsRef<JsValue>>::as_ref(&target).clone());
                e.prevent_default();
                navigate_with_transition(element.href().as_str());
            }
        }
    }) as Box<dyn FnMut(MouseEvent)>);
    document()
        .add_event_listener_with_callback("click", f.as_ref().unchecked_ref())
        .unwrap();
    f.forget();
}