#![allow(dead_code)]
#![warn(missing_debug_implementations)]

mod ui;
mod util;

#[cfg(feature = "webui")]
use crate::ui::{build_ui, init_ui};
#[cfg(feature = "webui")]
use solver::Solve;
#[cfg(feature = "webui")]
use ui::{controllers, editor::EditorController, models, sudoku::SudokuController, SudokuInfo};
#[cfg(feature = "webui")]
use util::Measure;

#[cfg(feature = "worker")]
use solver::Sudoku;

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

#[cfg(feature = "webui")]
#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    init_ui()?;
    Ok(())
}

#[wasm_bindgen]
#[cfg(feature = "webui")]
pub fn start() -> Result<(), JsValue> {
    build_ui()?;
    Ok(())
}

#[wasm_bindgen]
#[cfg(not(feature = "webui"))]
pub fn start() {}

#[wasm_bindgen]
#[cfg(feature = "webui")]
pub fn on_solve(solve: JsValue) -> Result<(), JsValue> {
    let solve: Solve = solve.into_serde().unwrap();
    SudokuController::on_solve(solve);
    Ok(())
}

#[wasm_bindgen]
#[cfg(feature = "webui")]
pub fn set_solver(f: &js_sys::Function) -> Result<(), JsValue> {
    controllers()
        .get::<SudokuController>("sudoku")?
        .set_solver(f);
    Ok(())
}

#[wasm_bindgen]
#[cfg(feature = "webui")]
pub fn on_measure(m: JsValue) -> Result<(), JsValue> {
    let m = m
        .into_serde::<Measure>()
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    models().get::<SudokuInfo>("info")?.set_measure(m);
    controllers().get::<EditorController>("editor")?.update()?;
    Ok(())
}

#[cfg(feature = "worker")]
#[wasm_bindgen]
pub fn solve(sudoku: &JsValue) -> Result<JsValue, JsValue> {
    let s: Sudoku = sudoku
        .into_serde()
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
    let solve = s.solve_steps();
    JsValue::from_serde(&solve).map_err(|e| JsValue::from_str(&format!("{}", e)))
}