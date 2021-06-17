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
        let mut sudoku = Sudoku::from(
            // "...6..8....35.4...65..217...6..............5..7138..2...7.1.6.4.1.......9....3..7",
            "....3.76.5....91.29.........49..53.......327...52..........75.4..1.4.....6.......",
        );

        let mut cages = solver::rules::Cages::default();
        cages.cells[14] = 1;
        cages.cells[23] = 1;
        cages.cells[24] = 1;
        cages.cells[25] = 1;
        cages.cages.push(26);


        cages.cells[47] = 2;
        cages.cells[48] = 2;
        cages.cells[56] = 2;
        cages.cells[57] = 2;
        cages.cages.push(17);


        cages.cells[42] = 3;
        cages.cells[43] = 3;
        cages.cages.push(9);

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
