use webelements::{we_builder, Result, WebElement};

use crate::ui::controller::app::AppController;
use crate::ui::controller::editor::EditorController;
use crate::ui::view::editor::Editor;
use crate::ui::view::info::Info;
use crate::ui::view::sudoku::Sudoku;
use crate::util::InitCell;

#[we_builder(
    <div class="app">
        <div class="app-main" we_field="main">
            <div class="sdk-box">
                <div class="sdk" we_field="sdk">
                    <div class="sdk-dummy" />
                    <Sudoku we_field="sudoku" we_element />
                    <Modal we_field="modal" we_element />
                </div>
            </div>
            <Editor we_field="editor" we_element />
            <Info we_field="info" we_element />
        </div>
        <div class="app-options"></div>
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct AppElement {}

impl AppElement {
    pub fn controller(&self) -> Result<InitCell<AppController>> {
        let app = AppController::build(self)?;
        Ok(app)
    }

    pub fn update(&self, app: &AppController) -> Result<()> {
        self.sudoku.update(&app.sudoku)?;
        self.modal.update(&app.editor)?;
        self.editor.update(&app.editor)?;
        self.info.update(&app.info)?;
        Ok(())
    }
}


#[we_builder(
    <div class="modal">
        <div class="progress-bar">
            <div class="progress" />
        </div>
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Modal {}

impl Modal {
    pub fn update(&self, editor: &EditorController) -> Result<()> {
        if !editor.disabled() {
            self.add_class("hidden");
        } else {
            self.remove_class("hidden");
        }
        Ok(())
    }
}