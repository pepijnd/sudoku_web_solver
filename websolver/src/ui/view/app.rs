use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlDivElement};

use crate::element;
use crate::util::ElementExt;

#[derive(Debug, Clone)]
pub struct AppElement {
    element: HtmlDivElement,
    main: HtmlDivElement,
    sdk: HtmlDivElement,
}

impl AppElement {
    pub fn new() -> Result<Self, JsValue> {
        let element = element!(div "app")?;
        let app_main = element!(div "app-main")?;
        element.append_child(&app_main)?;
        let sdk_box = element!(div "sdk-box")?;
        app_main.append_child(&sdk_box)?;
        let sdk = element!(div "sdk")?;
        sdk_box.append_child(&sdk)?;
        let dummy = element!(div "sdk-dummy")?;
        sdk.append_child(&dummy)?;
        let app_options = element!(div "app-options")?;
        element.append_child(&app_options)?;
        let element = Self {
            element,
            main: app_main,
            sdk,
        };
        Ok(element)
    }

    pub fn sdk(&self) -> &Element {
        &self.sdk
    }

    pub fn main(&self) -> &Element {
        &self.main
    }

    pub fn update(&self) -> Result<(), JsValue> {
        Ok(())
    }
}

impl AsRef<Element> for AppElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}
