use std::convert::TryInto;

use wasm_bindgen::JsValue;

use crate::ui::{
    controllers,
    editor::{EditorButtonAction, EditorElement},
    models,
    sudoku::{SudokuController, SudokuStateModel},
    Controller, SudokuInfo, UiController,
};

#[derive(Debug, Clone)]
pub struct EditorController {
    pub element: Option<EditorElement>,
}

impl EditorController {}

impl UiController for EditorController {
    type Element = EditorElement;

    fn update(&mut self) -> Result<(), JsValue> {
        if let Some(element) = &self.element {
            element.update()?
        }
        Ok(())
    }

    fn element(&self) -> Option<Self::Element> {
        self.element.clone()
    }

    fn set_element(&mut self, element: EditorElement) {
        self.element = Some(element);
    }

    fn build(self) -> Result<Controller<Self>, JsValue> {
        let element = EditorElement::new()?;
        let sudoku = controllers().get::<SudokuController>("sudoku").unwrap();
        let controller: Controller<Self> = self.into();
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        for btn in element.number_bar.buttons() {
            if let EditorButtonAction::SetValue(n) = btn.action() {
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
        let input = element.step_input.slider();
        let info = models().get::<SudokuInfo>("info").unwrap();
        element
            .step_input
            .slider()
            .on_input(Box::new(move |_event| {
                if let Some(value) = input.value() {
                    info.set_step(value.try_into().unwrap());
                    crate::util::g_update().unwrap();
                }
            }))?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl EditorController {
    pub fn button(action: EditorButtonAction) -> Result<(), JsValue> {
        let model = models().get::<SudokuStateModel>("sudoku")?;
        let info = models().get::<SudokuInfo>("info")?;
        let sudoku = controllers().get::<SudokuController>("sudoku")?;
        match action {
            EditorButtonAction::Solve => SudokuController::solve(),
            EditorButtonAction::Erase => {
                model.clear_state();
                info.clear_solve();
                crate::util::g_update().unwrap();
            }
            EditorButtonAction::Clear => {
                model.clear_start();
                model.clear_state();
                info.clear_solve();
                crate::util::g_update().unwrap();
            }
            EditorButtonAction::SetValue(n) => {
                if let Some(cell) = model.selected() {
                    model.start().set_cell(cell, n);
                    sudoku.update().unwrap();
                }
            }
            EditorButtonAction::First => {
                if info.solve().is_some() {
                    info.set_step(0);
                    crate::util::g_update().unwrap();
                }
            }
            EditorButtonAction::Prev => {
                if info.solve().is_some() {
                    let step = *info.step();
                    if step > 0 {
                        info.set_step(step - 1);
                        crate::util::g_update().unwrap();
                    }
                }
            }
            EditorButtonAction::Next => {
                if info.solve().is_some() {
                    let step = *info.step();
                    if step < *info.max() {
                        info.set_step(step + 1);
                        crate::util::g_update().unwrap();
                    }
                }
            }
            EditorButtonAction::Last => {
                if info.solve().is_some() {
                    let max = *info.max();
                    info.set_step(max);
                    crate::util::g_update().unwrap();
                }
            }
        }
        Ok(())
    }
}

impl Default for EditorController {
    fn default() -> Self {
        Self { element: None }
    }
}
