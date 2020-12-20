pub use we_derive::we_element;

pub trait WebElement {
    fn build() -> Self;
}

#[derive(Debug, Clone)]
pub struct Element {}

impl Element {
    pub fn new(name: impl AsRef<str>) -> Element {
        let _name = name;
        todo!()
    }

    pub fn append(&self, other: &Self) {
        let _other = other;
        todo!()
    }

    pub fn add_class(&self, class: impl AsRef<str>) {
        let _class = class;
        todo!()
    }

    pub fn set_attr(&self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let _key = key;
        let _value = value;
        todo!()
    }

    pub fn set_text(&self, text: impl AsRef<str>) {
        let _text = text;
        todo!()
    }
}

#[we_element(
    <div class="test">
        <p>this is a p</p>
        <div class="test another-test" atrr="testing">
            <span member="span">more testing</span>
        </div>
    </div>
)]
struct TestElement {}


#[cfg(test)]
mod tests {
    use super::{TestElement, WebElement};

    #[test]
    fn test() {
        TestElement::build();
    }
}
