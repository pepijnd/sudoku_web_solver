#![cfg(feature = "webui")]
use wasm_bindgen::prelude::*;

use crate::util::{InitCell, Measure};

use super::{controller::app::AppController, view::app::AppElement};
use solver::{Solve, Sudoku};
use webelements::{document, WebElementBuilder};

#[wasm_bindgen]
#[derive(Debug)]
pub struct App {
    controller: InitCell<AppController>,
    element: AppElement,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<App, JsValue> {
        let element = AppElement::build()?;
        let controller = element.controller()?;
        Ok(Self {
            controller,
            element,
        })
    }

    pub fn start(&self) -> Result<(), JsValue> {
        let sudoku = Sudoku::from(
            "8.1...9....927.....5....4.3.............5.....3............3......8...4..8.5.4...",
        );

        let cages = solver::rules::Cages {
            cages: vec![
                15, 7, 15, 25, 
                5, 13, 6, 12, 4, 
                11, 9, 8, 5, 
                12, 23, 33, 16, 
                12, 9, 
                1, 5, 12, 14, 22, 
                13, 18, 9, 8, 
                8, 25, 8, 
                6, 7, 9
                ],
            cells: [
                1, 1, 2, 2, 3, 3, 4, 4, 4, 
                5, 6, 6, 2, 3, 7, 4, 8, 9, 
                5, 10, 10, 11, 12, 13, 13, 8, 9, 
                14, 15, 15, 16, 16, 16, 16, 17, 17, 
                14, 15, 18, 18, 18, 16, 19, 19, 17,
                20, 21, 21, 22, 22, 16, 19, 23, 24,
                25, 26, 26, 22, 27, 28, 28, 23, 24,
                25, 26, 29, 30, 30, 30, 31, 31, 24,
                32, 26, 29, 33, 33, 30, 31, 34, 24
                ]
        };

        {
            let mut state = self.controller.sudoku.state.borrow_mut();
            state.set_start(sudoku);
            state.rules.cages = cages;
        }

        self.controller
            .info
            .info
            .borrow_mut()
            .update_properties()
            .unwrap();
        self.controller.update()?;
        document()?.body()?.append(&self.element)?;
        Ok(())
    }

    pub fn set_solver(&self, f: &js_sys::Function) {
        self.controller.sudoku.set_solver(f);
    }

    pub fn on_solve(&self, solve: JsValue) -> Result<(), JsValue> {
        let solve: Solve = solve.into_serde().unwrap();
        self.controller.sudoku.on_solve(solve)?;
        Ok(())
    }

    pub fn on_measure(&self, m: JsValue) -> Result<(), JsValue> {
        let m = m
            .into_serde::<Measure>()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        self.controller.info.info.borrow_mut().set_measure(m);
        self.controller.editor.update()?;
        Ok(())
    }
}
