use solver::Cell;
use webelements::{we_builder, Result, WebElement};

use crate::ui::controller::app::AppController;
use crate::ui::controller::sudoku::SudokuController;
use crate::util::InitCell;

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
            cell.set_bubble(None);
        }

        let model = sudoku.state.borrow();
        'cage: for (i, t) in model.rules.cages.cages.iter().enumerate() {
            for (c, n) in model.rules.cages.cells.iter().enumerate() {
                if i + 1 == *n as usize {
                    if let Some(cell) = self.cells.get(c) {
                        cell.set_bubble(Some(&format!("{}", t)));
                        continue 'cage;
                    }
                }
            }
        }

        Ok(())
    }
}

#[we_builder(
    <div class="sdk-cell">
        <div class="background" />
        <Indicator we_field="indicator" we_element />
        <Cage we_field="cage" we_element />
        <Options we_field="options" we_element />
        <Bubble we_field="bubble" we_element />
        <div class="sdk-number" we_field="number" />
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
        self.cage.cell = cell;
        self.options.cell = cell;
        self.indicator.cell = cell;
    }

    pub fn set_bubble(&self, content: Option<&str>) {
        // if content.is_some() {
        //     self.options.placeholder.remove_class("hidden");
        // } else {
        //     self.options.placeholder.add_class("hidden");
        // }
        self.bubble.set_content(content);
    }

    pub fn update(&self, sudoku: &SudokuController) {
        {
            let info = sudoku.app.info.info.lock().unwrap();
            let model = sudoku.state.borrow();
            let step = info
                .solve_step()
                .as_ref()
                .map(|s| *s.sudoku.cell(self.cell));
            let value = model.start().cell(self.cell);
            debug_assert!(value <= 9, "invalid cell value {}", value);
            self.number.remove_class("starting state empty");
            self.remove_class("target source selected");

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

            if let Some(step) = info.solve_step().as_ref() {
                if step.change.is_target(self.cell) {
                    self.add_class("target");
                } else if step.change.is_source(self.cell) {
                    self.add_class("source");
                }
            }

            if let Some(selected) = model.selected() {
                if selected == self.cell {
                    self.add_class("selected");
                }
            }
        }

        self.cage.update(sudoku);
        self.options.update(sudoku);
    }
}

#[we_builder(
    <div class="cell-indicator">
        <div class="indicator top" />
        <div class="indicator left" />
        <div class="indicator right" />
        <div class="indicator bottom" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Indicator {
    cell: Cell,
}

#[we_builder(
    <div class="cell-cage">
        <div class="cage" we_field="cage" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Cage {
    cell: Cell,
}

impl Cage {
    fn update(&self, sudoku: &SudokuController) {
        let state = sudoku.state.borrow();
        let cages = &state.rules.cages.cells;
        let id = cages[self.cell.index()];

        self.cage.remove_class("top left right bottom");

        if id == 0 {
            return;
        }

        if self.cell.row == 0 || cages[Cell::new(self.cell.row - 1, self.cell.col).index()] != id {
            self.cage.add_class("top");
        }
        if self.cell.row == 8 || cages[Cell::new(self.cell.row + 1, self.cell.col).index()] != id {
            self.cage.add_class("bottom");
        }
        if self.cell.col == 0 || cages[Cell::new(self.cell.row, self.cell.col - 1).index()] != id {
            self.cage.add_class("left");
        }
        if self.cell.col == 8 || cages[Cell::new(self.cell.row, self.cell.col + 1).index()] != id {
            self.cage.add_class("right");
        }
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
            cell.set_text(format!("{}", i + 1));
        }
        Ok(())
    }
}

impl Options {
    fn update(&self, sudoku: &SudokuController) {
        let info = sudoku.app.info.info.lock().unwrap();
        for (option, e) in self.options.iter().enumerate() {
            if let Some(step) = info.solve_step() {
                let index = option as u8 + 1;
                let cache = step.cache;
                e.remove_class("target");
                e.remove_class("source");
                e.remove_class("hidden");
                e.remove_class("digit");
                if !cache.cell(self.cell).has(index) {
                    e.add_class("hidden");
                }
                if let Some(step) = info.solve_step() {
                    if step.change.is_target_digit(self.cell, index) {
                        e.add_class("digit")
                    } else if step.change.is_target_option(self.cell, index) {
                        e.add_class("target")
                    } else if step.change.is_source_option(self.cell, index) {
                        e.add_class("source")
                    }
                }
            }
        }
    }
}

#[we_builder(
    <div class="bubble">
        <div class="bubble-bg">
            <div class="bubble-content" we_field="content" />
        </div>
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct Bubble {}

impl Bubble {
    pub fn set_content(&self, content: Option<&str>) {
        if let Some(content) = content {
            self.content.set_text(content);
            self.remove_class("hidden");
        } else {
            self.add_class("hidden");
        }
    }
}
