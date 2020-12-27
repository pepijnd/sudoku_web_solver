pub mod element;

use std::fmt::Display;

use wasm_bindgen::JsValue;

pub use we_derive::{we_builder, WebElement};
pub use element::{Element, WebElement, WebElementBuilder, elem};

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
    Cast(&'static str),
    Window,
    Document,
}

impl From<JsValue> for Error {
    fn from(from: JsValue) -> Self {
        Error::JsError(from)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsError(s) => {
                if let Some(s) = s.as_string() {
                    write!(f, "{}", s)
                } else {
                    Err(std::fmt::Error)
                }
            }
            Error::Cast(t) => writeln!(f, "unable to cast value to type `{}`", t),
            n => writeln!(f, "{:?}", n)
        }
    }
}

impl Error {
    pub fn as_jsvalue(&self) -> JsValue {
        if let Self::JsError(jsvalue) = self {
            jsvalue.clone()
        } else {
            JsValue::from_str(&self.to_string())
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

fn window() -> Result<web_sys::Window> {
    web_sys::window().ok_or(Error::Window)
}

fn document() -> Result<web_sys::Document> {
    window()?.document().ok_or(Error::Document)
}

#[allow(unused_unsafe)]
pub fn log(str: &str) {
    unsafe {
        web_sys::console::log_1(&JsValue::from_str(str));
    }
}