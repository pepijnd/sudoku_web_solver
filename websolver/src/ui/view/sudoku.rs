use solver::Cell;

use crate::ui::{model::sudoku::SudokuStateModel, models, SudokuInfo};

use webelements::{WebElement, we_builder};

#[we_builder(
    <div class="sdk-sudoku">
        <CellBox we_field="cells" we_repeat="81" we_element />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Sudoku {}

impl Sudoku {
    pub fn cells(&self) -> std::slice::Iter<CellBox> {
        self.cells.iter()
    }

    pub fn update(&self) {
        for cell in self.cells.iter() {
            cell.update();
        }
    }
}

#[we_builder(
    <div class="sdk-cell">
        <div class="sdk-number" we_field="number" />
        <Options we_field="options" we_element />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct CellBox {
    cell: Cell,
}

impl CellBox {
    pub fn cell(&self) -> Cell {
        self.cell
    }

    pub fn update(&self) {
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        let info = models().get::<SudokuInfo>("info").unwrap();
        let step = info
            .solve_step()
            .as_ref()
            .map(|s| *s.sudoku.cell(self.cell));

        let value = model.start().cell(self.cell);
        debug_assert!(value <= 9, format!("invalid cell value {}", value));
        self.number.remove_class("starting state empty");
        self.remove_class("selected");

        self.remove_class("target");
        self.remove_class("source");
        if let Some(step) = info.solve_step().as_ref() {
            if step.change.is_target(self.cell) {
                self.add_class("target");
            } else if step.change.is_source(self.cell) {
                self.add_class("source");
            }
        }

        if info.solve().is_some() {
            self.options.remove_class("hidden");
        } else {
            self.options.add_class("hidden");
        }
        if value > 0 {
            self.number.set_text(&format!("{}", value));
            self.number.add_class("starting");
            self.options.add_class("hidden");
        } else if let Some(value) = step {
            self.number.add_class("state");
            if value > 0 {
                self.number.set_text(&format!("{}", value));
                self.options.add_class("hidden");
            } else {
                self.number.set_text("");
            }
        } else {
            self.number.add_class("empty");
            self.number.set_text("");
        }
        if let Some(selected) = model.selected() {
            if selected == self.cell {
                self.add_class("selected");
            }
        }
        self.options.update();
    }
}

#[we_builder(
    <div class="cell-options">
        <div class="cell-option" we_field="options" we_repeat="9" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
struct Options {
    cell: Cell
}

impl Options {
    fn update(&self) {
        let info = models().get::<SudokuInfo>("info").unwrap();
        for (option, e) in self.options.iter().enumerate() {
            if let Some(step) = info.solve_step().as_ref() {
                let index = option as u8 + 1;
                let mut cache = step.cache;
                e.remove_class("target");
                e.remove_class("source");
                if cache.options(self.cell, &step.sudoku).has(index) {
                    e.remove_class("hidden");
                } else {
                    e.add_class("hidden");
                }
                if let Some(step) = info.solve_step().as_ref() {
                    if step.change.is_target_option(self.cell, index) {
                        e.remove_class("hidden");
                        e.add_class("target")
                    } else if step.change.is_source_option(self.cell, index) {
                        e.remove_class("hidden");
                        e.add_class("source")
                    }
                }
            }
        }
    }
}