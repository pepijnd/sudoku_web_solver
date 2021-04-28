#![cfg(feature = "webui")]

mod build;
mod controller;
mod model;
mod view;

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

pub use build::App;
pub use view::common::*;
