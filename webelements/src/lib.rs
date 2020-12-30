pub mod element;

use std::{fmt::Display, ops::Deref};

use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

pub use we_derive::{we_builder, WebElement};
pub use element::{Element, WebElement, WebElementBuilder, elem};
use web_sys::KeyboardEvent;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
    Cast(&'static str),
    Window,
    Document,
    Value,
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

pub struct Window {
    window: web_sys::Window
}

impl Deref for Window {
    type Target = web_sys::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

pub fn window() -> Result<Window> {
    Ok(Window {
        window: web_sys::window().ok_or(Error::Window)?
    })
}

pub struct Document {
    document: web_sys::Document
}

impl Document {
    pub fn on_key(&self, mut callback: impl FnMut(KeyboardEvent) + 'static ) -> Result<()> {
        let closure = Closure::wrap(Box::new( move |e| {
            callback(e)
        }) as Box<dyn FnMut(KeyboardEvent)>);
        self.document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .map_err(Error::JsError)?;
        closure.forget();
        Ok(())
    }
}

impl Deref for Document {
    type Target = web_sys::Document;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

pub fn document() -> Result<Document> {
    Ok(Document{
        document: window()?.document().ok_or(Error::Document)?
    })
}

#[allow(unused_unsafe)]
pub fn log(str: &str) {
    unsafe {
        web_sys::console::log_1(&JsValue::from_str(str));
    }
}