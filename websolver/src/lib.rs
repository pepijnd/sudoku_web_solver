#![allow(dead_code)]
#![warn(missing_debug_implementations)]
#![feature(result_flattening)]

mod ui;
mod util;

use wasm_bindgen::prelude::*;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}
