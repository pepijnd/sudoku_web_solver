use webelements::{WebElement, we_builder, WebElementBuilder, Result};

use crate::ui::ButtonElement;
use crate::ui::{editor::EditorAction, models, SudokuInfo};
use crate::{ui::editor::EditorController};

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
    pub fn update(&self) -> Result<()> {
        self.steps.update()
    }
}

#[we_builder(
    <div class="btn-panel sudoku-numbers">
        <EditorButton we_field="buttons" we_repeat="10" we_element />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct NumberBar {}

#[we_builder(
    <div class="btn-panel solve-options">
        <EditorButton we_field="solve" we_element />
        <EditorButton we_field="erase" we_element />
        <EditorButton we_field="clear" we_element />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct OptionBar {}

#[we_builder(
    <ButtonElement class="sdk-btn" we_element />
)]
#[derive(Debug, Clone, WebElement)]
pub struct EditorButton {
    action: EditorAction
}

impl EditorButton {
    pub fn connect(&self) -> Result<()> {
        let action = self.action;
        self.on_click(move |_event| {
            EditorController::on_action(action).unwrap()
        })
    }

    pub fn update(&self) {
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
        <button we_field="first" />
        <button we_field="prev" />
        <StepSlider we_field="slider" we_element />
        <button we_field="next" />
        <button we_field="last" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct StepInput {}

impl StepInput {
    pub fn update(&self) -> Result<()> {
        self.slider.update()
    }
}

#[we_builder(
    <div class="step-slider">
        <div class="slider-bubble-track">
            <div class="slider-bubble" />
        </div>
        <input type="range" we_field="slider" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct StepSlider {}

impl StepSlider {
    fn update(&self) -> Result<()> {
        let info = models().get::<SudokuInfo>("info")?;
        if info.solve().is_none() {
            self.slider.set_min(-1);
            self.slider.set_max(1);
            self.slider.set_value(0);
            self.slider.set_attr("disabled", "true")?;
        } else {
            self.slider.set_min(0);
            self.slider.set_max(info.max());
            self.slider.set_value(info.step());
            self.slider.del_attr("disabled")?;
        }
        Ok(())
    }
}