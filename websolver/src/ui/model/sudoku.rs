use solver::{rules::Rules, Cell, Sudoku};

#[derive(Debug)]
pub struct SudokuStateModel {
    pub start: SudokuModel,
    pub state: Option<SudokuModel>,
    pub rules: Rules,
    selected: Option<Cell>,
}

impl SudokuStateModel {
    pub fn start(&self) -> &SudokuModel {
        &self.start
    }

    pub fn start_mut(&mut self) -> &mut SudokuModel {
        &mut self.start
    }

    pub fn set_start(&mut self, start: Sudoku) {
        self.start.set(start)
    }

    pub fn clear_start(&mut self) {
        self.start.clear()
    }

    pub fn state(&self) -> Option<&SudokuModel> {
        self.state.as_ref()
    }

    pub fn set_state(&mut self, sudoku: SudokuModel) {
        self.state.replace(sudoku);
    }

    pub fn clear_state(&mut self) {
        self.state.take();
    }

    pub fn selected(&self) -> Option<Cell> {
        self.selected
    }

    pub fn set_selected(&mut self, cell: Cell) {
        self.selected.replace(cell);
    }

    pub fn deselect(&mut self) {
        self.selected.take();
    }
}

impl Default for SudokuStateModel {
    fn default() -> Self {
        Self {
            start: Default::default(),
            state: None,
            rules: Rules::default(),
            selected: None,
        }
    }
}

impl From<Sudoku> for SudokuStateModel {
    fn from(start: Sudoku) -> Self {
        Self {
            start: SudokuModel::from(start),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
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

impl SudokuModel {
    pub fn set(&mut self, sudoku: Sudoku) {
        self.sudoku = sudoku;
    }

    pub fn get(&self) -> &Sudoku {
        &self.sudoku
    }

    pub fn cell(&self, cell: Cell) -> u8 {
        *self.sudoku.cell(cell)
    }

    pub fn set_cell(&mut self, cell: Cell, value: u8) {
        self.sudoku.set_cell(cell, value);
    }

    pub fn clear(&mut self) {
        self.sudoku = Sudoku::default()
    }
}
