use webelements::{we_builder, Result, WebElement};

use crate::{ui::{controller::editor::EditorController, editor::{EditorAction, EditorMode}}, util::InitCell};

#[we_builder(
    <div>
        <p class="editor header">Options</p>
        <p class="editor option">Rules:</p>
        <div class="editor rules">
            <Rule we_field="default" we_element />   
            <Rule we_field="cages" we_element />   
        </div>
    </div>
)]
#[derive(Debug, Clone)]
pub struct Rules {}

impl WebElement for Rules {
    fn init(&mut self) -> Result<()> {
        self.default.set_mode(EditorMode::Default);
        self.cages.set_mode(EditorMode::Cages);
        Ok(())
    }
}

impl Rules {
    pub fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        self.default.connect(InitCell::clone(&editor))?;
        self.cages.connect(InitCell::clone(&editor))?;
        Ok(())
    }

    pub fn update(&self, editor: &EditorController) -> Result<()> {
        self.default.update(editor)?;
        self.cages.update(editor)?;
        Ok(())
    }
}

#[we_builder(
    <div class="editor rule"> 
        <div class="editor config-rule" we_field="rule" />
        <div class="editor clear-rule" we_field="clear" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Rule {
    mode: EditorMode,
}

impl Rule {
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        let mode = self.mode;
        self.on_click(move |_| {
            editor.on_action(EditorAction::SetMode(mode)).unwrap()
        })?;
        Ok(())
    }

    pub fn update(&self, editor: &EditorController) -> Result<()> {
        if editor.mode() == self.mode {
            self.add_class("active")
        } else {
            self.remove_class("active")
        }
        self.rule.set_text(match self.mode {
            EditorMode::Default => "Normal",
            EditorMode::Cages => "Killer Sudoku",
        });
        Ok(())
    }
}
