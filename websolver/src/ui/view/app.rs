use webelements::{WebElement, we_builder, Result};

use crate::ui::view::sudoku::Sudoku;
use crate::ui::view::editor::Editor;

#[we_builder(
    <div class="app">
        <div class="app-main" we_field="main">
            <div class="sdk-box">
                <div class="sdk" we_field="sdk">
                    <div class="sdk-dummy" />
                    <Sudoku we_field="sudoku" we_element />
                </div>
            </div>
            <Editor we_field="editor" we_element />
        </div>
        <div class="app-options"></div>
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct AppElement {}

impl AppElement {
    pub fn update(&self) -> Result<()> {
        Ok(())
    }
}