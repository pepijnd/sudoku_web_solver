use webelements::{we_builder, WebElement};

#[we_builder(
    <button />
)]
#[derive(Debug, Clone, WebElement)]
pub struct ButtonElement {}

impl ButtonElement {}
