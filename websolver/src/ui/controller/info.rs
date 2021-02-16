use crate::ui::info::Info;

use webelements::{Result, WebElementBuilder};

#[derive(Debug, Clone)]
pub struct InfoController {
    pub element: Option<Info>,
}

impl InfoController {}

impl InfoController {

    fn update(&mut self) -> Result<()> {
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

    fn build(self) -> Result<Controller<Self>> {
        let controller: Controller<Self> = self.into();
        let element = Info::build()?;
        controller.set_element(element);
        Ok(controller)
    }
}

impl Default for InfoController {
    fn default() -> Self {
        Self { element: None }
    }
}
