use js_sys::wasm_bindgen::closure::Closure;
use js_sys::wasm_bindgen::JsValue;
use sycamore::rt::NodeRef;

pub trait NodeRefAsHtmlElementExt {
    fn as_html_element<T>(&self) -> T
    where
        T: From<JsValue>;
}

impl NodeRefAsHtmlElementExt for NodeRef {
    fn as_html_element<T>(&self) -> T
    where
        T: From<JsValue>,
    {
        let node = self.get();
        let js: &JsValue = node.as_ref();
        T::from(js.clone())
    }
}

pub trait FromFn {
    fn from_fn(f: impl FnMut() + 'static) -> Self;
}

impl FromFn for Closure<dyn FnMut() + 'static> {
    fn from_fn(f: impl FnMut() + 'static) -> Closure<dyn FnMut() + 'static> {
        Closure::wrap(Box::new(f) as Box<dyn FnMut() + 'static>)
    }
}