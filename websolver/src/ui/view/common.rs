use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement, HtmlElement, MouseEvent};

use crate::util::document;

#[derive(Debug, Clone)]
pub struct ButtonElement {
    pub element: HtmlButtonElement,
}

impl ButtonElement {
    pub fn new() -> Result<Self, JsValue> {
        let element = document()?
            .create_element("button")?
            .dyn_into::<web_sys::HtmlButtonElement>()?;

        Ok(Self { element })
    }

    pub fn on_click(&self, closure: Box<dyn FnMut(MouseEvent)>) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}

impl AsRef<HtmlElement> for ButtonElement {
    fn as_ref(&self) -> &HtmlElement {
        &self.element
    }
}

impl AsRef<Element> for ButtonElement {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}
