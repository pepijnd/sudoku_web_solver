use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement, HtmlInputElement, InputEvent, MouseEvent};

use crate::ui::ButtonElement;
use crate::ui::{editor::EditorButtonAction, models, SudokuInfo};
use crate::util::ElementExt;
use crate::{element, ui::editor::EditorController};

#[derive(Debug, Clone)]
pub struct EditorElement {
    pub element: HtmlDivElement,
    pub number_bar: ButtonBarElement,
    pub option_bar: ButtonBarElement,
    pub step_input: StepInputElement,
}

impl EditorElement {
    pub fn new() -> Result<Self, JsValue> {
        let element = element!()?;
        let number_bar = ButtonBarElement::number_bar()?;
        let option_bar = ButtonBarElement::option_bar()?;
        let step_input = StepInputElement::new()?;
        element.append_child(number_bar.as_ref())?;
        element.append_child(option_bar.as_ref())?;
        element.append_child(step_input.as_ref())?;
        Ok(Self {
            element,
            number_bar,
            option_bar,
            step_input,
        })
    }

    pub fn update(&self) -> Result<(), JsValue> {
        self.step_input.update()
    }
}

impl AsRef<Element> for EditorElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct ButtonBarElement {
    element: HtmlDivElement,
    buttons: Vec<EditorButtonElement>,
}

impl ButtonBarElement {
    pub fn buttons(&self) -> std::slice::Iter<EditorButtonElement> {
        self.buttons.iter()
    }

    pub fn number_bar() -> Result<Self, JsValue> {
        let element = element!(div "btn-panel sudoku-numbers")?;
        let buttons = (0..9)
            .map(|n| EditorButtonElement::new(EditorButtonAction::SetValue(n)))
            .collect::<Result<Vec<EditorButtonElement>, JsValue>>()?;
        for btn in buttons.iter() {
            element.append_child(btn.as_ref())?;
        }
        Ok(Self { element, buttons })
    }

    pub fn option_bar() -> Result<Self, JsValue> {
        let element = element!(div "btn-panel solve-options")?;
        element.add_class("btn-panel");
        let buttons = vec![
            EditorButtonElement::new(EditorButtonAction::Solve)?,
            EditorButtonElement::new(EditorButtonAction::Erase)?,
            EditorButtonElement::new(EditorButtonAction::Clear)?,
        ];
        for btn in buttons.iter() {
            element.append_child(btn.as_ref())?;
        }
        Ok(Self { element, buttons })
    }
}

impl AsRef<Element> for ButtonBarElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct EditorButtonElement {
    element: ButtonElement,
    action: EditorButtonAction,
}

impl EditorButtonElement {
    pub fn new(action: EditorButtonAction) -> Result<Self, JsValue> {
        let element = ButtonElement::new()?;
        element.add_class("sdk-btn");
        let btn = Self { element, action };
        btn.connect()?;
        btn.update();
        Ok(btn)
    }

    pub fn connect(&self) -> Result<(), JsValue> {
        let action = self.action;
        self.on_click(Box::new(move |_event| {
            EditorController::button(action).unwrap()
        }))
    }

    pub fn update(&self) {
        let text = self.action.to_string();
        self.set_text(&text);
    }

    pub fn action(&self) -> EditorButtonAction {
        self.action
    }

    pub fn on_click(&self, closure: Box<dyn FnMut(MouseEvent)>) -> Result<(), JsValue> {
        self.element.on_click(closure)
    }
}

impl AsRef<Element> for EditorButtonElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct StepInputElement {
    element: HtmlDivElement,
    first: EditorButtonElement,
    prev: EditorButtonElement,
    slider: StepSliderInputElement,
    next: EditorButtonElement,
    last: EditorButtonElement,
}

impl StepInputElement {
    pub fn new() -> Result<Self, JsValue> {
        let element = element!(div "step-actions")?;
        let first = EditorButtonElement::new(EditorButtonAction::First)?;
        let prev = EditorButtonElement::new(EditorButtonAction::Prev)?;
        let slider = StepSliderInputElement::new()?;
        let next = EditorButtonElement::new(EditorButtonAction::Next)?;
        let last = EditorButtonElement::new(EditorButtonAction::Last)?;
        element.append_child(first.as_ref())?;
        element.append_child(prev.as_ref())?;
        element.append_child(slider.as_ref())?;
        element.append_child(next.as_ref())?;
        element.append_child(last.as_ref())?;
        Ok(Self {
            element,
            first,
            prev,
            slider,
            next,
            last,
        })
    }

    pub fn slider(&self) -> StepSliderInputElement {
        self.slider.clone()
    }

    pub fn update(&self) -> Result<(), JsValue> {
        self.slider().update()
    }
}

impl AsRef<Element> for StepInputElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct StepSliderInputElement {
    element: HtmlDivElement,
    bubble: HtmlDivElement,
    slider: HtmlInputElement,
}

impl StepSliderInputElement {
    fn new() -> Result<Self, JsValue> {
        let element = element!(div "step-slider")?;
        let bubble_hold = element!(div "slider-bubble-track")?;
        let bubble = element!(div "slider-bubble")?;
        let slider = element!(input "slider-input")?;
        slider.set_type("range");
        bubble_hold.append_child(&bubble)?;
        element.append_child(&bubble_hold)?;
        element.append_child(&slider)?;
        let step = Self {
            element,
            bubble,
            slider,
        };
        step.update()?;
        Ok(step)
    }

    pub fn value(&self) -> Option<i32> {
        self.slider.value().parse::<i32>().ok()
    }

    fn update(&self) -> Result<(), JsValue> {
        let info = models().get::<SudokuInfo>("info")?;
        if info.solve().is_none() {
            self.slider.set_min("-1");
            self.slider.set_max("1");
            self.slider.set_value("0");
            self.slider.set_attribute("disabled", "true")?;
        } else {
            self.slider.set_min("0");
            self.slider.set_max(&format!("{}", info.max()));
            self.slider.set_value(&format!("{}", info.step()));
            self.slider.remove_attribute("disabled")?;
        }
        Ok(())
    }

    pub fn on_input(&self, closure: Box<dyn FnMut(InputEvent)>) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.slider
            .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}

impl AsRef<Element> for StepSliderInputElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}
