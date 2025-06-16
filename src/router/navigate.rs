use js_sys::wasm_bindgen::JsCast;
use js_sys::wasm_bindgen::closure::Closure;
use sycamore::prelude::document;

pub fn navigate(route: &str) {
    sycamore_router::navigate(route)
}

pub fn navigate_with_transition(route: &str) {
    let route = route.to_string();
    let f = Closure::wrap(Box::new(move || navigate(route.as_str())) as Box<dyn FnMut()>);
    document()
        .start_view_transition_with_update_callback(Some(f.as_ref().unchecked_ref()))
        .unwrap();
    f.forget();
}
