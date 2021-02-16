use std::convert::TryInto;

use lazy_static::lazy_static;

use crate::ui::{
    editor::{EditorAction, Editor},
    sudoku::{SudokuController, SudokuStateModel},
    SudokuInfo
};

use webelements::{Result, WebElementBuilder};

#[derive(Debug, Clone)]
pub struct EditorController {}


impl EditorController {
    fn get() -> &'static mut EditorController {
        lazy_static! {
            static ref EDITORCONTROLLER: EditorController = EditorController {};
        }

        &mut EDITORCONTROLLER
    }

    fn update(&mut self) -> Result<()> {
        if let Some(element) = &self.element {
            element.update()?
        }
        Ok(())
    }

    fn element(&self) -> Option<Self::Element> {
        self.element.clone()
    }

    fn set_element(&mut self, element: Editor) {
        self.element = Some(element);
    }

    fn build(self) -> Result<Controller<Self>> {
        let element = Self::Element::build()?;
        let sudoku = controllers().get::<SudokuController>("sudoku").unwrap();
        let controller: Controller<Self> = self.into();
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        for btn in element.numbers.buttons {
            if let EditorAction::SetValue(n) = btn.action() {
                let sudoku = sudoku.clone();
                let model = model.clone();
                btn.on_click(Box::new(move |_event| {
                    if let Some(cell) = model.selected() {
                        {
                            model.start().set_cell(cell, n);
                            sudoku.update().unwrap();
                        }
                    }
                }))?;
                btn.update();
            }
        }
        let input = element.steps.slider.slider;
        let info = models().get::<SudokuInfo>("info").unwrap();
        input.on_input(Box::new(move |_event| {
            if let Ok(value) = input.get_value::<i32>() {
                info.set_step(value.try_into().unwrap());
                crate::util::global_update().unwrap();
            }
        }))?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl EditorController {
    pub fn on_action(action: EditorAction) -> Result<()> {
        let model = models().get::<SudokuStateModel>("sudoku")?;
        let info = models().get::<SudokuInfo>("info")?;
        let sudoku = controllers().get::<SudokuController>("sudoku")?;
        match action {
            EditorAction::Solve => SudokuController::solve(),
            EditorAction::Erase => {
                model.clear_state();
                info.clear_solve();
                crate::util::global_update().unwrap();
            }
            EditorAction::Clear => {
                model.clear_start();
                model.clear_state();
                info.clear_solve();
                crate::util::global_update().unwrap();
            }
            EditorAction::SetValue(n) => {
                if let Some(cell) = model.selected() {
                    model.start().set_cell(cell, n);
                    sudoku.update().unwrap();
                }
            }
            EditorAction::First => {
                if info.solve().is_some() {
                    info.set_step(0);
                    crate::util::global_update().unwrap();
                }
            }
            EditorAction::Prev => {
                if info.solve().is_some() {
                    let step = *info.step();
                    if step > 0 {
                        info.set_step(step - 1);
                        crate::util::global_update().unwrap();
                    }
                }
            }
            EditorAction::Next => {
                if info.solve().is_some() {
                    let step = *info.step();
                    if step < *info.max() {
                        info.set_step(step + 1);
                        crate::util::global_update().unwrap();
                    }
                }
            }
            EditorAction::Last => {
                if info.solve().is_some() {
                    let max = *info.max();
                    info.set_step(max);
                    crate::util::global_update().unwrap();
                }
            }

            _ => {}
        }
        Ok(())
    }
}

impl Default for EditorController {
    fn default() -> Self {
        Self { element: None }
    }
}
