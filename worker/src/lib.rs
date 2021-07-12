#![allow(dead_code)]
#![warn(missing_debug_implementations)]

use wasm_bindgen::prelude::*;
use webelements::Scope;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Worker {
    scope: Scope,
}

#[wasm_bindgen]
impl Worker {
    #[wasm_bindgen(constructor)]
    pub fn new(scope: JsValue) -> Result<Worker, JsValue> {
        let worker = Self {
            scope: Scope::new(scope)?,
        };
        let worker_ref = worker.clone();
        worker.scope.set_onmessage(move |value| {
            worker_ref.on_message(value);
        })?;
        Ok(worker)
    }
}

impl Worker {
    fn on_message(&self, value: JsValue) {
        webelements::log(format!("{:?}", value));
    }

    fn post_message(&self, message: JsValue) -> Result<(), JsValue> {
        self.scope.post_message(message)?;
        Ok(())
    }
}
