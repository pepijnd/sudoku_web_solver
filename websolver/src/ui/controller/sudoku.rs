use solver::Solve;
use wasm_bindgen::JsValue;

use crate::ui::{
    controllers, models,
    sudoku::{Sudoku, SudokuModel, SudokuStateModel},
    Controller, SudokuInfo, UiController,
};

use webelements::{Result, WebElementBuilder};

#[derive(Debug, Clone)]
pub struct SudokuController {
    pub solver: Option<js_sys::Function>,
}

impl Default for SudokuController {
    fn default() -> Self {
        Self {
            solver: None,
        }
    }
}

impl Controller<SudokuController> {
    pub fn solver(&self) -> Option<js_sys::Function> {
        self.borrow().solver.clone()
    }

    pub fn set_solver(&self, solver: &js_sys::Function) {
        self.borrow_mut().solver = Some(solver.clone())
    }
}

impl UiController for SudokuController {
    type Element = Sudoku;

    fn update(&mut self) -> Result<()> {
        if let Some(element) = self.element.as_ref() {
            element.update();
        }
        Ok(())
    }
    fn build(self) -> Result<Controller<Self>> {
        let controller: Controller<Self> = self.into();
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        let closure = {
            let controller = controller.clone();
            let model = model.clone();
            webelements::document()?.on_key(move |event| {
                let selected = { model.selected() };
                if let Some(mut selected) = selected {
                    match &*event.key() {
                        "ArrowLeft" => {
                            if selected.col > 0 {
                                selected.col -= 1
                            }
                        }
                        "ArrowUp" => {
                            if selected.row > 0 {
                                selected.row -= 1
                            }
                        }
                        "ArrowRight" => {
                            if selected.col < 8 {
                                selected.col += 1
                            }
                        }
                        "ArrowDown" => {
                            if selected.row < 8 {
                                selected.row += 1
                            }
                        }
                        "Delete" => {
                            model.start().set_cell(selected, 0);
                        }
                        str => {
                            if let Ok(value) = str.parse::<u8>() {
                                if value <= 9 {
                                    model.start().set_cell(selected, value);
                                }
                            }
                        }
                    }
                    model.set_selected(selected);
                    controller.update().unwrap();
                }
            });
        };
        let element = {
            let element = Self::Element::build()?;
            for cell in element.cells() {
                let controller = controller.clone();
                let model = model.clone();
                let clicked = cell.cell();
                cell.on_click(Box::new(move |_event| {
                    model.set_selected(clicked);
                    controller.update().unwrap();
                }))?;
            }
            element
        };
        controller.set_element(element);
        controller.update().unwrap();
        Ok(controller)
    }
}

impl SudokuController {
    pub fn solve() {
        let controller = controllers().get::<SudokuController>("sudoku").unwrap();
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        let sudoku = model.start().get();
        if let Some(solver) = controller.solver() {
            let this = JsValue::null();
            solver
                .call1(&this, &JsValue::from_serde(&sudoku).unwrap())
                .unwrap();
        }
    }

    pub fn on_solve(solve: Solve) {
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        let info = models().get::<SudokuInfo>("info").unwrap();

        let step = solve.iter().last().unwrap();
        model.set_state(SudokuModel::from(step.sudoku).into());
        info.set_solve(solve);
        let max = *info.max();
        info.set_step(max);
        crate::util::global_update().unwrap();
    }
}
