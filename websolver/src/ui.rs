mod build;
mod controller;
mod model;
mod view;

pub mod sudoku {
    pub use super::{controller::sudoku::*, model::sudoku::*, view::sudoku::*};
}

pub mod editor {
    pub use super::{controller::editor::*, model::editor::*, view::editor::*};
}

pub mod app {
    pub use super::{controller::app::*, view::app::*};
}

pub mod info {
    pub use super::{controller::info::*, view::info::*};
}

pub use model::info::SudokuInfo;

pub use build::App;
pub use view::common::*;
