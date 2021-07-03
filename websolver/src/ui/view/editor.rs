use webelements::{we_builder, Result, WebElement, WebElementBuilder};

use crate::ui::controller::app::AppController;
use crate::ui::editor::{EditorAction, EditorController};
use crate::ui::ButtonElement;
use crate::util::InitCell;

#[we_builder(
    <div>
        <NumberBar we_field="numbers" we_element />
        <OptionBar we_field="options" we_element />
        <StepInput we_field="steps" we_element />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Editor {}

impl Editor {
    pub fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        self.numbers.connect(InitCell::clone(&editor))?;
        self.options.connect(InitCell::clone(&editor))?;
        self.steps.connect(InitCell::clone(&editor))?;
        Ok(())
    }

    pub fn controller(&self, app: InitCell<AppController>) -> Result<EditorController> {
        EditorController::build(app, self)
    }

    pub fn update(&self, editor: &EditorController) -> Result<()> {
        self.numbers.update(editor);
        self.options.update(editor);
        self.steps.update(editor)?;
        Ok(())
    }
}

#[we_builder(
    <div class="btn-panel sudoku-numbers">
        <EditorButton we_field="buttons" we_repeat="10" we_element />
    </div>
)]
#[derive(Debug, Clone)]
pub struct NumberBar {}

impl WebElement for NumberBar {
    fn init(&mut self) -> Result<()> {
        for (n, btn) in self.buttons.iter_mut().enumerate() {
            btn.action = EditorAction::SetValue(n as u8);
        }
        Ok(())
    }
}

impl NumberBar {
    fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        for btn in self.buttons.iter() {
            btn.connect(InitCell::clone(&editor))?;
        }
        Ok(())
    }

    fn update(&self, editor: &EditorController) {
        for btn in self.buttons.iter() {
            btn.update(editor);
        }
    }
}

#[we_builder(
    <div class="btn-panel solve-options">
        <EditorButton we_field="solve" we_element />
        <EditorButton we_field="erase" we_element />
        <EditorButton we_field="clear" we_element />
    </div>
)]
#[derive(Debug, Clone)]
pub struct OptionBar {}

impl WebElement for OptionBar {
    fn init(&mut self) -> Result<()> {
        self.solve.action = EditorAction::Solve;
        self.erase.action = EditorAction::Erase;
        self.clear.action = EditorAction::Clear;
        Ok(())
    }
}

impl OptionBar {
    pub fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        self.solve.connect(InitCell::clone(&editor))?;
        self.erase.connect(InitCell::clone(&editor))?;
        self.clear.connect(InitCell::clone(&editor))?;
        Ok(())
    }

    pub fn update(&self, editor: &EditorController) {
        self.solve.update(editor);
        self.erase.update(editor);
        self.clear.update(editor);
    }
}

#[we_builder(
    <ButtonElement class="sdk-btn" we_element />
)]
#[derive(Debug, Clone, WebElement)]
pub struct EditorButton {
    action: EditorAction,
}

impl EditorButton {
    pub fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        let action = self.action;
        self.on_click(move |_event| editor.on_action(action).unwrap())
    }

    pub fn update(&self, _editor: &EditorController) {
        let text = self.action.to_string();
        self.set_text(&text);
    }

    pub fn action(&self) -> EditorAction {
        self.action
    }

    pub fn set_action(&mut self, action: EditorAction) {
        self.action = action
    }
}

#[we_builder(
    <div class="step-actions">
        <EditorButton we_field="first" we_element />
        <EditorButton we_field="prev" we_element />
        <StepSlider we_field="slider" we_element />
        <EditorButton we_field="next" we_element />
        <EditorButton we_field="last" we_element />
    </div>
)]
#[derive(Debug, Clone)]
pub struct StepInput {}

impl WebElement for StepInput {
    fn init(&mut self) -> Result<()> {
        self.first.action = EditorAction::First;
        self.prev.action = EditorAction::Prev;
        self.next.action = EditorAction::Next;
        self.last.action = EditorAction::Last;
        Ok(())
    }
}

impl StepInput {
    pub fn connect(&self, editor: InitCell<EditorController>) -> Result<()> {
        self.first.connect(InitCell::clone(&editor))?;
        self.prev.connect(InitCell::clone(&editor))?;
        self.next.connect(InitCell::clone(&editor))?;
        self.last.connect(InitCell::clone(&editor))?;
        Ok(())
    }

    pub fn update(&self, editor: &EditorController) -> Result<()> {
        self.first.update(editor);
        self.prev.update(editor);
        self.slider.update(editor)?;
        self.next.update(editor);
        self.last.update(editor);
        Ok(())
    }
}

#[we_builder(
    <div class="step-slider">
        <div class="slider-bubble-track">
            <div class="slider-bubble" />
        </div>
        <input class="slider-input" type="range" we_field="slider" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct StepSlider {}

impl StepSlider {
    fn update(&self, editor: &EditorController) -> Result<()> {
        let info = editor.app.info.info.borrow();
        if info.solve().is_none() {
            self.slider.set_min(-1);
            self.slider.set_max(1);
            self.slider.set_value(0);
            self.slider.set_attr("disabled", "true")?;
        } else {
            self.slider.set_min(0);
            self.slider.set_max(info.max());
            self.slider.set_value(info.step());
            webelements::log(format!("{:?}", info.step()));
            self.slider.del_attr("disabled")?;
        }
        Ok(())
    }
}
