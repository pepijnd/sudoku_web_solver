use webelements::Result;

use crate::{ui::view::app::AppElement, util::InitCell};

use super::{editor::EditorController, info::InfoController, sudoku::SudokuController};

#[derive(Debug, Clone)]
pub struct AppController {
    element: AppElement,
    pub editor: InitCell<EditorController>,
    pub info: InitCell<InfoController>,
    pub sudoku: InitCell<SudokuController>,
}

impl AppController {
    pub fn build(element: &AppElement) -> Result<InitCell<Self>> {
        let app = InitCell::with(AppController {
            element: element.clone(),
            editor: InitCell::new(),
            info: InitCell::new(),
            sudoku: InitCell::new(),
        });

        InitCell::init(
            &app.sudoku,
            app.element.sudoku.controller(InitCell::clone(&app))?,
        );
        InitCell::init(
            &app.info,
            app.element.info.controller(InitCell::clone(&app))?,
        );
        InitCell::init(
            &app.editor,
            app.element.editor.controller(InitCell::clone(&app))?,
        );

        Ok(app)
    }

    pub fn update(&self) -> Result<()> {
        self.element.update(self)?;
        Ok(())
    }
}
