use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, HtmlButtonElement, HtmlElement, MouseEvent, Node, Window};

use serde::Deserialize;

#[cfg(feature = "webui")]
use crate::ui::{app::AppController, controllers, Controller, UiController};

pub trait NodeExt: Sized {
    fn deep_clone(&self) -> Result<Self, JsValue>;
}

impl<T> NodeExt for T
where
    T: AsRef<Node> + JsCast,
{
    fn deep_clone(&self) -> Result<Self, JsValue> {
        let node: &Node = self.as_ref();
        let node = node.clone_node_with_deep(true)?.dyn_into::<T>()?;
        Ok(node)
    }
}

#[cfg(feature = "webui")]
pub trait ElementExt {
    fn has_class(&self, class: &str) -> bool;
    fn toggle_class(&self, class: &str);
    fn add_class(&self, class: &str);
    fn remove_class(&self, class: &str);
    fn set_text(&self, str: &str);
    fn append(&self, e: &impl AsRef<Element>) -> Result<(), JsValue>;
    fn build(&self, e: &Controller<impl UiController>) -> Result<(), JsValue>;
}

#[cfg(feature = "webui")]
impl<E: AsRef<Element>> ElementExt for E {
    fn has_class(&self, class: &str) -> bool {
        let e = self.as_ref();
        let class_string: String = e.class_name();
        for class_name in class_string.split_whitespace() {
            if class == class_name {
                return true;
            }
        }
        false
    }

    fn toggle_class(&self, class: &str) {
        for class in class.split_whitespace() {
            if self.has_class(class) {
                self.remove_class(class);
            } else {
                self.add_class(class);
            }
        }
    }

    fn add_class(&self, class: &str) {
        for class in class.split_whitespace() {
            if !self.has_class(class) {
                let e = self.as_ref();
                let mut class_string: String = e.class_name();
                class_string.push_str(&format!(" {}", class));
                e.set_class_name(&class_string);
            }
        }
    }

    fn remove_class(&self, class: &str) {
        for class in class.split_whitespace() {
            if self.has_class(class) {
                let e = self.as_ref();
                let class_string = e.class_name();
                let mut new_string = Vec::<&str>::new();
                for class_name in class_string.split_whitespace() {
                    if class_name != class {
                        new_string.push(class_name)
                    }
                }
                let new_string = new_string.join(" ");
                e.set_class_name(&new_string);
            }
        }
    }

    fn set_text(&self, text: &str) {
        self.as_ref().set_inner_html(text)
    }

    fn append(&self, e: &impl AsRef<Element>) -> Result<(), JsValue> {
        self.as_ref().append_child(e.as_ref()).map(|_| ())
    }

    fn build(&self, e: &Controller<impl UiController>) -> Result<(), JsValue> {
        if let Some(e) = e.element() {
            self.as_ref().append_child(e.as_ref()).map(|_| ())?
        }
        Ok(())
    }
}

pub type Shared<T> = Rc<RefCell<T>>;

#[allow(unused_unsafe)]
pub fn log(str: &str) {
    unsafe {
        web_sys::console::log_1(&JsValue::from_str(str));
    }
}

pub enum KeyCode {
    Left,
    Up,
    Right,
    Down,
    Other,
}

impl From<u32> for KeyCode {
    fn from(value: u32) -> Self {
        match value {
            37 => Self::Left,
            38 => Self::Up,
            39 => Self::Right,
            40 => Self::Down,

            _ => Self::Other,
        }
    }
}

pub fn document() -> Result<Document, JsValue> {
    let window: Window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Unable to get Window"))?;
    let document: Document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Unable to get Document"))?;
    Ok(document)
}

pub fn body() -> Result<HtmlElement, JsValue> {
    let body = document()?
        .body()
        .ok_or_else(|| JsValue::from_str("Unable to get Body"))?;
    Ok(body)
}

pub fn get_element(id: &str) -> Result<Element, JsValue> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("Unable to find Element"))
}

pub fn make_button(closure: Box<dyn FnMut(MouseEvent)>) -> Result<HtmlButtonElement, JsValue> {
    let val = document()?
        .create_element("button")?
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut(_)>);
    val.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(val)
}

pub fn measure<T, R>(mut measure: T) -> Result<(Measure, R), JsValue>
where
    T: FnMut() -> R,
{
    let perf = web_sys::window()
        .ok_or_else(|| JsValue::from_str(""))?
        .performance()
        .ok_or_else(|| JsValue::from_str(""))?;
    perf.mark("marker_start").unwrap();
    let out = measure();
    perf.mark("marker_end").unwrap();
    perf.measure_with_start_mark_and_end_mark("measure", "marker_start", "marker_end")?;
    let measure = perf
        .get_entries_by_name("measure")
        .iter()
        .last()
        .ok_or_else(|| JsValue::from_str(""))?;
    perf.clear_measures_with_measure_name("measure");
    perf.clear_marks_with_mark_name("marker_start");
    perf.clear_marks_with_mark_name("marker_end");
    Ok((
        measure
            .into_serde::<Measure>()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?,
        out,
    ))
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
    name: String,
    start_time: f64,
    duration: f64,
}

impl std::fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} ms", self.duration)
    }
}

#[cfg(feature = "webui")]
pub fn g_update() -> Result<(), JsValue> {
    controllers().get::<AppController>("app")?.update()?;
    Ok(())
}

#[macro_export]
macro_rules! element {
    () => {
        element!(div)
    };
    ($e:ident) => {{
        let e = stringify!($e);
        let element = crate::util::document().unwrap().create_element(e).unwrap();
        macro_rules! dyn_element {
            (div) => {
                element.dyn_into::<web_sys::HtmlDivElement>()
            };
            (input) => {
                element.dyn_into::<web_sys::HtmlInputElement>()
            };
            (button) => {
                element.dyn_into::<web_sys::HtmlButtonElement>()
            };
            (span) => {
                element.dyn_into::<web_sys::HtmlSpanElement>()
            };
        };
        dyn_element!($e)
    }};
    ($e:ident $c:expr) => {{
        let element = element!($e);
        for e in &element {
            e.add_class($c);
        }
        element
    }};
}
