use lazy_static::lazy_static;

use crate::ui::{editor::EditorController, info::InfoController, sudoku::SudokuController};

use webelements::{Result, WebElementBuilder};

#[derive(Debug, Clone)]
pub struct AppController {
}

impl AppController {
    fn get() -> &'static mut AppController {
        lazy_static! {
            static ref APPCONTROLLER: AppController = AppController {};
        }

        &mut APPCONTROLLER
    }

    fn update(&mut self) -> Result<()> {
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

    fn build(self) -> Result<crate::ui::Controller<Self>> {
        let controller: Controller<Self> = self.into();
        let element = Self::Element::build()?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self { element: None }
    }
}
