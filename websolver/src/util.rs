use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use serde::Deserialize;

#[cfg(feature = "webui")]
use crate::ui::{app::AppController, controllers, Controller, UiController};

use webelements::Result;

pub type Shared<T> = Rc<RefCell<T>>;

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


// pub fn measure<T, R>(mut measure: T) -> Result<(Measure, R), JsValue>
// where
//     T: FnMut() -> R,
// {
//     let perf = web_sys::window()
//         .ok_or_else(|| JsValue::from_str(""))?
//         .performance()
//         .ok_or_else(|| JsValue::from_str(""))?;
//     perf.mark("marker_start").unwrap();
//     let out = measure();
//     perf.mark("marker_end").unwrap();
//     perf.measure_with_start_mark_and_end_mark("measure", "marker_start", "marker_end")?;
//     let measure = perf
//         .get_entries_by_name("measure")
//         .iter()
//         .last()
//         .ok_or_else(|| JsValue::from_str(""))?;
//     perf.clear_measures_with_measure_name("measure");
//     perf.clear_marks_with_mark_name("marker_start");
//     perf.clear_marks_with_mark_name("marker_end");
//     Ok((
//         measure
//             .into_serde::<Measure>()
//             .map_err(|e| JsValue::from_str(&format!("{}", e)))?,
//         out,
//     ))
// }

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
pub fn global_update() -> Result<()> {
    controllers().get::<AppController>("app")?.update()?;
    Ok(())
}