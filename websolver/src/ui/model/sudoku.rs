use solver::{Cell, Sudoku};

use crate::ui::{Model, UiModel};

#[derive(Debug, Clone)]
pub struct SudokuStateModel {
    start: Model<SudokuModel>,
    state: Option<Model<SudokuModel>>,
    selected: Option<Cell>,
}

impl UiModel for SudokuStateModel {}

impl Model<SudokuStateModel> {
    pub fn start(&self) -> Model<SudokuModel> {
        self.borrow().start.clone()
    }

    pub fn set_start(&self, start: Sudoku) {
        self.borrow_mut().start.set(start)
    }

    pub fn clear_start(&self) {
        self.borrow_mut().start.clear()
    }

    pub fn state(&self) -> Option<Model<SudokuModel>> {
        self.borrow().state.clone()
    }

    pub fn set_state(&self, sudoku: Model<SudokuModel>) {
        self.borrow_mut().state = Some(sudoku);
    }

    pub fn clear_state(&self) {
        self.borrow_mut().state = None;
    }

    pub fn selected(&self) -> Option<Cell> {
        self.borrow().selected
    }

    pub fn set_selected(&self, cell: Cell) {
        self.borrow_mut().selected = Some(cell);
    }

    pub fn deselect(&self) {
        self.borrow_mut().selected = None
    }
}

impl Default for SudokuStateModel {
    fn default() -> Self {
        Self {
            start: Default::default(),
            state: None,
            selected: None,
        }
    }
}

impl From<Sudoku> for SudokuStateModel {
    fn from(start: Sudoku) -> Self {
        Self {
            start: SudokuModel::from(start).into(),
            state: None,
            selected: None,
        }
    }
}

#[derive(Debug)]
pub struct SudokuModel {
    sudoku: Sudoku,
}

impl Default for SudokuModel {
    fn default() -> Self {
        Self {
            sudoku: Sudoku::default(),
        }
    }
}

impl From<Sudoku> for SudokuModel {
    fn from(sudoku: Sudoku) -> Self {
        Self { sudoku }
    }
}

impl UiModel for SudokuModel {}

impl Model<SudokuModel> {
    pub fn set(&self, sudoku: Sudoku) {
        self.borrow_mut().sudoku = sudoku;
    }

    pub fn get(&self) -> Sudoku {
        self.borrow().sudoku
    }

    pub fn cell(&self, cell: Cell) -> u8 {
        *self.borrow().sudoku.cell(cell)
    }

    pub fn set_cell(&self, cell: Cell, value: u8) {
        self.borrow_mut().sudoku.set_cell(cell, value);
    }

    pub fn clear(&mut self) {
        self.borrow_mut().sudoku = Sudoku::default()
    }
}
