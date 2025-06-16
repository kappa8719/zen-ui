use js_sys::wasm_bindgen::closure::Closure;
use js_sys::wasm_bindgen::JsCast;
use sycamore::prelude::document;
use sycamore_router::{HistoryIntegration, Integration};
use web_sys::MouseEvent;

pub struct TransitionedHistoryIntegration {
    _inner: HistoryIntegration,
}

impl TransitionedHistoryIntegration {
    pub fn new() -> TransitionedHistoryIntegration {
        Self {
            _inner: HistoryIntegration::new(),
        }
    }
}

impl Integration for TransitionedHistoryIntegration {
    fn current_pathname(&self) -> String {
        self._inner.current_pathname()
    }

    fn on_popstate(&self, f: Box<dyn FnMut()>) {
        let f = Closure::wrap(f);
        let closure = Box::new(move || {
            document()
                .start_view_transition_with_update_callback(Some(f.as_ref().unchecked_ref()))
                .unwrap();
        }) as Box<dyn FnMut()>;

        self._inner.on_popstate(closure);
    }

    fn click_handler(&self) -> Box<dyn Fn(MouseEvent)> {
        self._inner.click_handler()
    }
}
