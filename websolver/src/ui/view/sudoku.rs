use solver::Cell;

use crate::{
    ui::{
        controller::{app::AppController, sudoku::SudokuController},
    },
    util::InitCell,
};

use webelements::{we_builder, Result, WebElement};

#[we_builder(
    <div class="sdk-sudoku">
        <CellBox we_field="cells" we_repeat="81" we_element />
    </div>
)]
#[derive(Debug, Clone)]
pub struct Sudoku {}

impl WebElement for Sudoku {
    fn init(&mut self) -> Result<()> {
        for (index, cell) in self.cells.iter_mut().enumerate() {
            cell.set_cell(Cell::from_index(index));
        }
        Ok(())
    }
}

impl Sudoku {
    pub fn controller(&self, app: InitCell<AppController>) -> Result<SudokuController> {
        SudokuController::build(app, self)
    }

    pub fn cells(&self) -> std::slice::Iter<CellBox> {
        self.cells.iter()
    }

    pub fn update(&self, sudoku: &SudokuController) -> Result<()> {
        for cell in self.cells.iter() {
            cell.update(sudoku);
        }
        Ok(())
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

    pub fn set_cell(&mut self, cell: Cell) {
        self.cell = cell;
        self.options.cell = cell;
    }

    pub fn update(&self, sudoku: &SudokuController) {
        let info = sudoku.app.info.info.borrow();
        let model = sudoku.state.borrow();
        let step = info
            .solve_step()
            .as_ref()
            .map(|s| *s.sudoku.cell(self.cell));
        let value = model.start().cell(self.cell);
        debug_assert!(value <= 9, "invalid cell value {}", value);
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
        self.options.update(sudoku);
    }
}

#[we_builder(
    <div class="cell-options">
        <div class="cell-option" we_field="options" we_repeat="9" />
    </div>
)]
#[derive(Debug, Clone)]
pub struct Options {
    cell: Cell,
}

impl WebElement for Options {
    fn init(&mut self) -> Result<()> {
        dbg!("{:?}", &self.options);
        for (i, cell) in self.options.iter().enumerate() {
            cell.set_text(format!("{}", i+1));
        }
        Ok(())
    }
}

impl Options {
    fn update(&self, sudoku: &SudokuController) {
        let info = sudoku.app.info.info.borrow();
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
