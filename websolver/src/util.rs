use std::{cell::UnsafeCell, rc::Rc};

use lazy_static::__Deref;
use serde::Deserialize;

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

pub struct InitCell<T> {
    value: Rc<UnsafeCell<Option<T>>>,
}

impl<T> std::ops::Deref for InitCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: we never give out any &mut references to `this.value` so immutable access is safe.
        let this = unsafe { &*self.value.get() };
        if let Some(value) = &this {
            value
        } else {
            panic!("value not initialized")
        }
    }
}

impl<T> InitCell<T> {
    pub fn new() -> Self {
        Self {
            value: Rc::new(UnsafeCell::new(None)),
        }
    }

    pub fn with(value: T) -> Self {
        Self {
            value: Rc::new(UnsafeCell::new(Some(value))),
        }
    }

    pub fn init(this: &Self, value: T) {
        {
            // SAFETY: we never give out any &mut references to `this.value` so immutable access is safe.
            let cell = unsafe { &*this.value.get() };
            if cell.is_some() {
                panic!("initial value already set");
            }
        }
        // SAFETY: `this.value` was `None`, we never give a reference until it is set therefore we can safely get an `&mut Option<T>` in this scope
        let cell = unsafe { &mut *this.value.get() };
        cell.replace(value);
    }
}

impl<T> std::fmt::Debug for InitCell<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T> Default for InitCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for InitCell<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
        }
    }
}
