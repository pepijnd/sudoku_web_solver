use crate::ui::{app::AppElement, controllers, Controller, UiController};

use crate::ui::{editor::EditorController, info::InfoController, sudoku::SudokuController};

#[derive(Debug, Clone)]
pub struct AppController {
    pub element: Option<AppElement>,
}

impl AppController {}

impl UiController for AppController {
    type Element = AppElement;

    fn update(&mut self) -> Result<(), wasm_bindgen::JsValue> {
        if let Some(element) = &self.element {
            element.update()?;
        }

        controllers().get::<InfoController>("info")?.update()?;
        controllers().get::<SudokuController>("sudoku")?.update()?;
        controllers().get::<EditorController>("editor")?.update()?;

        Ok(())
    }

    fn element(&self) -> Option<Self::Element> {
        self.element.clone()
    }

    fn set_element(&mut self, element: Self::Element) {
        self.element = Some(element)
    }

    fn build(self) -> Result<crate::ui::Controller<Self>, wasm_bindgen::JsValue> {
        let controller: Controller<Self> = self.into();
        let element = AppElement::new()?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self { element: None }
    }
}
