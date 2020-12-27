use elem::ElemTy;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{document, Error, Result};


pub mod elem {
    use wasm_bindgen::JsCast;
    use we_derive::{element_types};
    pub trait ElemTy {
        type Elem: AsRef<web_sys::Element>;
        fn make() -> crate::Result<Self::Elem>;
    }
    element_types!();
}

pub trait WebElementBuilder<E> where E: ElemTy {
    fn build() -> Result<Self>
    where
        Self: std::marker::Sized;

    fn root(&self) -> &Element<E>;
}

pub trait WebElement<E>: WebElementBuilder<E> where E: ElemTy {
    fn init(&mut self);
}

#[derive(Debug, Clone)]
pub struct Element<E> where E: ElemTy {
    element: E::Elem,
}

impl<E> AsRef<Element<E>> for Element<E> where E: ElemTy {
    fn as_ref(&self) -> &Element<E> {
        self.root()
    }
}

impl<E> Element<E> where E: ElemTy {
    pub fn new() -> Result<Element<E>> {
        let element = E::make()?;
        Ok(Self { element })
    }

    fn as_element(&self) -> &web_sys::Element {
        &self.element.as_ref()
    }

    fn as_node(&self) -> &web_sys::Node {
        &self.element.as_ref()
    }

    pub fn append(&self, other: impl AsRef<Element<E>>) -> Result<()> {
        self.element.as_ref().append_child(other.as_ref().as_node())?;
        Ok(())
    }

    pub fn append_list(&self, items: impl IntoIterator<Item=impl AsRef<Element<E>>>) -> Result<()> {
        items.into_iter().try_for_each(|i| self.append(i))
    }

    pub fn root(&self) -> &Element<E> {
        &self
    }

    pub fn set_as_body(&self) -> Result<()> {
        let element = self
            .as_element()
            .clone()
            .dyn_into::<HtmlElement>()
            .map_err(|_| Error::Cast(std::any::type_name::<HtmlElement>()))?;
        document()?.set_body(Some(&element));
        Ok(())
    }

    pub fn has_class(&self, class: impl AsRef<str>) -> bool {
        let class_string: String = self.as_element().class_name();
        for class_name in class_string.split_whitespace() {
            if class.as_ref() == class_name {
                return true;
            }
        }
        false
    }

    pub fn toggle_class(&self, class: impl AsRef<str>) {
        for class in class.as_ref().split_whitespace() {
            if self.has_class(class) {
                self.remove_class(class);
            } else {
                self.add_class(class);
            }
        }
    }

    pub fn add_class(&self, class: impl AsRef<str>) {
        for class in class.as_ref().split_whitespace() {
            if !self.has_class(class) {
                let mut class_string: String = self.as_element().class_name();
                class_string.push_str(&format!(" {}", class));
                self.as_element().set_class_name(&class_string);
            }
        }
    }

    pub fn remove_class(&self, class: impl AsRef<str>) {
        for class in class.as_ref().split_whitespace() {
            if self.has_class(class) {
                let class_string = self.as_element().class_name();
                let mut new_string = Vec::<&str>::new();
                for class_name in class_string.split_whitespace() {
                    if class_name != class {
                        new_string.push(class_name)
                    }
                }
                let new_string = new_string.join(" ");
                self.as_element().set_class_name(&new_string);
            }
        }
    }

    pub fn set_text(&self, text: impl AsRef<str>) {
        self.as_element().set_inner_html(text.as_ref())
    }

    pub fn set_attr(&self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<()> {
        self.as_element()
            .set_attribute(name.as_ref(), value.as_ref())?;
        Ok(())
    }
}
