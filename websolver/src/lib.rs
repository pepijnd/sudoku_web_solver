#![allow(dead_code)]
#![warn(missing_debug_implementations)]

mod ui;
mod util;

#[cfg(feature = "worker")]
use solver::Sudoku;

use solver::{Config, rules::Rules};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[cfg(feature = "worker")]
#[wasm_bindgen]
pub fn solve(sudoku: &JsValue, rules: &JsValue) -> Result<JsValue, JsValue> {
    let sudoku: Sudoku = sudoku
        .into_serde()
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    let rules: Rules = rules
        .into_serde()
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let mut config = Config {
        rules, ..Default::default()
    };
    config.add_rules_solvers();
    let solve = sudoku.solve_steps(Some(config));
    JsValue::from_serde(&solve).map_err(|e| JsValue::from_str(&format!("{}", e)))
}
