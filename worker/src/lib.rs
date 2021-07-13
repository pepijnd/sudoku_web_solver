#![allow(dead_code)]
#![warn(missing_debug_implementations)]

use solver::{solving::Reporter, threading::ThreadMessage};
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
        worker.post_message(JsValue::from_serde(&ThreadMessage::Ready).unwrap())?;
        Ok(worker)
    }
}

impl Worker {
    fn on_message(&self, value: JsValue) {
        match value.into_serde::<ThreadMessage>() {
            Ok(msg) => match msg {
                ThreadMessage::Job(job) => {
                    let (config, job) = *job;
                    let worker = self.clone();
                    let mut reported = 0.0;
                    let solve = job.solve(
                        &config,
                        Reporter::new(Box::new(move |p| {
                            if p > reported + (0.01 / 8.0) {
                                worker
                                    .post_message(
                                        JsValue::from_serde(&ThreadMessage::Progress(p)).unwrap(),
                                    )
                                    .unwrap();
                                reported = p;
                            }
                        })),
                    );
                    self.post_message(JsValue::from_serde(&ThreadMessage::Result(solve)).unwrap())
                        .unwrap();
                }
                other => {
                    webelements::log!(
                        "Invalid Message type: ",
                        JsValue::from_serde(&other).unwrap()
                    )
                }
            },
            Err(e) => (webelements::log!(JsValue::from_str(&e.to_string()))),
        }
    }

    fn post_message(&self, message: JsValue) -> Result<(), JsValue> {
        self.scope.post_message(message)?;
        Ok(())
    }
}
