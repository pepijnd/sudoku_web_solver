use wasm_bindgen::JsValue;

use crate::ui::{info::InfoElement, Controller, UiController};

#[derive(Debug, Clone)]
pub struct InfoController {
    pub element: Option<InfoElement>,
}

impl InfoController {}

impl UiController for InfoController {
    type Element = InfoElement;

    fn update(&mut self) -> Result<(), JsValue> {
        if let Some(element) = &self.element {
            element.update()?;
        }
        Ok(())
    }

    fn element(&self) -> Option<Self::Element> {
        self.element.clone()
    }

    fn set_element(&mut self, element: Self::Element) {
        self.element = Some(element);
    }

    fn build(self) -> Result<Controller<Self>, JsValue> {
        let controller: Controller<Self> = self.into();
        let element = InfoElement::new()?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl Default for InfoController {
    fn default() -> Self {
        Self { element: None }
    }
}
