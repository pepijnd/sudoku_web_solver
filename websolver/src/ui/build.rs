use solver::{Solve, Sudoku};
use wasm_bindgen::prelude::*;
use webelements::{document, WebElementBuilder};

use super::controller::app::AppController;
use super::view::app::AppElement;
use crate::util::{InitCell, Measure};

#[wasm_bindgen]
#[derive(Debug, Clone)]
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

    pub fn on_worker_msg(&self, msg: JsValue) {
        webelements::log(format!("{:?}", msg));
    }

    pub fn start(&self, worker: JsValue) -> Result<(), JsValue> {
        let worker = webelements::Worker::new(worker)?;

        let app = self.clone();
        let worker_ref = worker.clone();
        worker.set_onmessage(move |value| {
            app.on_worker_msg(value);
            worker_ref
                .post_message(&JsValue::from_str("hi from main thread"))
                .unwrap();
        })?;

        let sudoku = Sudoku::from(
            ".................................................................................",
        );

        let cages = solver::rules::Cages {
            cages: vec![20, 27, 26, 24, 28, 17, 18, 30, 16, 24],
            cells: [
                0, 0, 0, 0, 1, 2, 2, 2, 3, 0, 0, 0, 0, 1, 1, 1, 2, 3, 0, 0, 0, 0, 4, 4, 5, 5, 3, 0,
                0, 0, 0, 0, 4, 4, 5, 3, 6, 7, 8, 0, 0, 0, 4, 5, 3, 6, 7, 8, 8, 0, 0, 0, 0, 0, 6, 7,
                7, 8, 8, 0, 0, 0, 0, 6, 9, 10, 10, 10, 0, 0, 0, 0, 6, 9, 9, 9, 10, 0, 0, 0, 0,
            ],
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

    pub fn on_progress(&self, p: JsValue) -> Result<(), JsValue> {
        let p = p
            .into_serde::<Vec<(u32, u32)>>()
            .map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        self.controller.info.info.borrow_mut().set_progress(p)?;
        self.controller.info.update()?;
        Ok(())
    }
}
