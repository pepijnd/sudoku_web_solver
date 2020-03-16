use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Element, HtmlDivElement, HtmlSpanElement};

use crate::util::ElementExt;
use crate::{
    element,
    ui::{models, SudokuInfo},
};

#[derive(Debug, Clone)]
pub struct InfoElement {
    element: HtmlDivElement,
    stats: Vec<InfoStatsElement>,
}

impl InfoElement {
    pub fn new() -> Result<Self, JsValue> {
        let element = element!(div "solve-info")?;
        let stats = vec![
            InfoStatsElement::new("Tech", "tech")?,
            InfoStatsElement::new("Steps", "steps")?,
            InfoStatsElement::new("Total Guess Steps", "guess_steps")?,
            InfoStatsElement::new("Guesses", "guess")?,
            InfoStatsElement::new("Total Guesses", "guess_all")?,
        ];
        for stat in &stats {
            element.append_child(stat.as_ref())?;
        }
        let element = Self { element, stats };
        element.update()?;
        Ok(element)
    }

    pub fn update(&self) -> Result<(), JsValue> {
        for stat in &self.stats {
            stat.update()?;
        }
        Ok(())
    }
}

impl AsRef<Element> for InfoElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct InfoStatsElement {
    key: &'static str,
    element: HtmlDivElement,
    value: HtmlSpanElement,
}

impl InfoStatsElement {
    pub fn new(label: &str, key: &'static str) -> Result<Self, JsValue> {
        let element = element!(div "solve-stat")?;
        let info_label = element!(span "info-label")?;
        info_label.set_inner_text(&format!("{}: ", &label));
        let value = element!(span "info-value")?;
        element.append_child(&info_label)?;
        element.append_child(&value)?;
        let element = Self {
            key,
            element,
            value,
        };
        element.update()?;
        Ok(element)
    }

    pub fn update(&self) -> Result<(), JsValue> {
        let model = models().get::<SudokuInfo>("info")?;
        if let Some(value) = model.property(self.key) {
            self.value.set_inner_text(&value);
        }
        Ok(())
    }
}

impl AsRef<Element> for InfoStatsElement {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}
