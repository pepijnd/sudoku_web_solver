use webelements::{WebElement, we_builder};

#[we_builder(
    <button />
)]
#[derive(Debug, Clone, WebElement)]
pub struct ButtonElement {}

impl ButtonElement {}