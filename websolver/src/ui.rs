#![cfg(feature = "webui")]

use wasm_bindgen::JsValue;
use web_sys::Element;

use lazy_static::lazy_static;

mod build;
mod controller;
mod model;
mod resources;
mod view;

use resources::{DynItem, DynItemValue, DynMap};

pub mod sudoku {
    pub use super::controller::sudoku::*;
    pub use super::model::sudoku::*;
    pub use super::view::sudoku::*;
}

pub mod editor {
    pub use super::controller::editor::*;
    pub use super::model::editor::*;
    pub use super::view::editor::*;
}

pub mod app {
    pub use super::controller::app::*;
    pub use super::view::app::*;
}

pub mod info {
    pub use super::controller::info::*;
    pub use super::view::info::*;
}

pub use model::info::SudokuInfo;

pub use build::{build_ui, init_ui};
pub use view::common::*;

pub trait UiModel: std::fmt::Debug + Default {}

pub trait UiController: std::fmt::Debug + Default {
    type Element: AsRef<Element>;

    fn update(&mut self) -> Result<(), JsValue>;
    fn element(&self) -> Option<Self::Element>;
    fn set_element(&mut self, element: Self::Element);
    fn build(self) -> Result<Controller<Self>, JsValue>;
}

impl<T> Controller<T>
where
    T: UiController,
{
    pub fn update(&self) -> Result<(), JsValue> {
        self.borrow_mut().update()
    }
    pub fn element(&self) -> Option<T::Element> {
        self.borrow().element()
    }
    pub fn set_element(&self, element: T::Element) {
        self.borrow_mut().set_element(element);
    }
    pub fn append<E: AsRef<Element>>(&self, element: &E) -> Result<(), JsValue> {
        if let Some(e) = self.element() {
            e.as_ref().append_child(element.as_ref()).map(|_| ())
        } else {
            Err(JsValue::from_str(
                "can't append element, controller not build",
            ))
        }
    }
}

impl<T> Model<T> where T: UiModel {}

impl<T> DynItemValue for T where T: std::fmt::Debug + Default {}

pub type Model<T> = DynItem<T>;
pub type Controller<T> = DynItem<T>;

lazy_static! {
    static ref MODELS: DynMap = DynMap::new();
    static ref CONTROLLERS: DynMap = DynMap::new();
}

pub fn models() -> DynMap {
    MODELS.clone()
}

pub fn controllers() -> DynMap {
    CONTROLLERS.clone()
}

#[cfg(feature = "webui")]
#[cfg(test)]
mod tests {
    use crate::ui::{models, sudoku::SudokuStateModel, Model};
    use solver::Cell;

    #[test]
    fn test() {
        let sudoku: Model<SudokuStateModel> = SudokuStateModel::default().into();
        sudoku.set_selected(Cell::new(1, 1));
        models().insert("sudoku", sudoku);
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        assert_eq!(model.selected(), Some(Cell::new(1, 1)));
    }
}
