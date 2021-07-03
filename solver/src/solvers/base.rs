use crate::solving::{CellMod, Entry, StateMod};
use crate::{AdvanceResult, Cell, CellOptions, EntrySolver, Solver, State};

#[derive(Debug, Copy, Clone)]
pub struct StateInit;

impl EntrySolver for StateInit {
    fn advance(&mut self, state: &mut State) -> AdvanceResult {
        state.info.push_state();
        AdvanceResult::Advance
    }
}

impl Default for StateInit {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateNoOp;

impl EntrySolver for StateNoOp {
    fn advance(&mut self, _state: &mut State) -> AdvanceResult {
        AdvanceResult::Advance
    }
}

impl Default for StateNoOp {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateSolved;

impl EntrySolver for StateSolved {
    fn advance(&mut self, _state: &mut State) -> AdvanceResult {
        AdvanceResult::Invalid
    }

    fn terminate(&self) -> bool {
        true
    }
}

impl Default for StateSolved {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateIncomplete;

impl EntrySolver for StateIncomplete {
    fn advance(&mut self, _state: &mut State) -> AdvanceResult {
        AdvanceResult::Invalid
    }

    fn terminate(&self) -> bool {
        true
    }
}

impl Default for StateIncomplete {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateInvalid;

impl EntrySolver for StateInvalid {
    fn advance(&mut self, state: &mut State) -> AdvanceResult {
        state.info.correct = false;
        AdvanceResult::Invalid
    }

    fn terminate(&self) -> bool {
        false
    }
}

impl Default for StateInvalid {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BaseSolver;

impl EntrySolver for BaseSolver {
    fn advance(&mut self, state: &mut State) -> AdvanceResult {
        let mut solved = true;
        let mut mods = StateMod::from(state.info.tech);
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell::new(row, col);
                let value = *state.sudoku.cell(cell);
                if value == 0 {
                    let options = state.options.options(cell, &state.sudoku);
                    if let Some(value) = options.found() {
                        state.update(cell, value);
                        mods.push_target(CellMod::digit(cell, value));
                    } else if options.is_empty() {
                        return AdvanceResult::Invalid;
                    } else {
                        solved = false;
                    }
                }
            }
        }
        if mods.has_targets() {
            state.info.push_mod(mods);
        }
        state.info.solved = solved;
        AdvanceResult::Advance
    }
}

impl Default for BaseSolver {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Backtrace {
    cell: Option<Cell>,
    options: CellOptions,
    job: bool,
}

impl EntrySolver for Backtrace {
    fn advance(&mut self, state: &mut State) -> AdvanceResult {
        state.info.correct = false;
        if self.job {
            return AdvanceResult::Advance;
        }
        if self.cell.is_none() {
            if let Some(cell) = Self::heuristic(state) {
                self.cell.replace(cell);
                self.options = state.options.options(cell, &state.sudoku)
            } else {
                return AdvanceResult::Invalid;
            }
        }
        if let Some(max_depth) = state.config.max_threading_depth {
            let cell = self.cell.expect("target cell should be set at this point");
            if state.info.depth < max_depth.get() {
                let mut jobs = Vec::new();
                for value in self.options.iter() {
                    let mut state = state.clone();
                    let mods = StateMod::from_change(state.info.tech, cell, value);
                    state.info.push_mod(mods);
                    state.update(cell, value);
                    state.info.depth += 1;
                    jobs.push(Entry {
                        state,
                        solver: Solver::BackTrace,
                        entry: Box::new(Self {
                            cell: Some(cell),
                            options: CellOptions::default(),
                            job: true,
                        }),
                    })
                }
                return AdvanceResult::Split(jobs);
            }
        }
        if let Some(value) = self.options.take() {
            let cell = self.cell.expect("target cell should be set at this point");
            state.update(cell, value);
            let mods = StateMod::from_change(state.info.tech, cell, value);
            state.info.push_mod(mods);
            AdvanceResult::Advance
        } else {
            AdvanceResult::Invalid
        }
    }

    fn verified(&self) -> bool {
        self.job
    }
}

impl Backtrace {
    pub fn heuristic(state: &mut State) -> Option<Cell> {
        let mut candidate: Option<(usize, Cell)> = None;
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell::new(row, col);
                if *state.sudoku.cell(cell) != 0 {
                    continue;
                };
                let options = state.options.options(cell, &state.sudoku);
                let mut score = 10 - options.len();
                if state.config.rules.cages.cells[cell.index()] != 0 {
                    score *= 2;
                }
                if candidate.is_none() || score > candidate.unwrap().0 {
                    candidate.replace((score, cell));
                }
            }
        }
        candidate.map(|c| c.1)
    }
}

impl Default for Backtrace {
    fn default() -> Self {
        Self {
            cell: None,
            options: CellOptions::all(),
            job: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Backtrace;
    use crate::{AdvanceResult, Cell, CellOptions, EntrySolver, State, Sudoku};

    static SAMPLE: &str =
        "...6..8....35.4...65..217...6..............5..7138..2...7.1.6.4.1.......9....3..7";

    #[test]
    fn backtrace_test() {
        let sudoku = Sudoku::from(SAMPLE);
        let mut state = State {
            sudoku,
            ..Default::default()
        };
        let mut solver = Backtrace::default();
        assert_eq!(
            state.options.options(Cell::new(0, 5), &state.sudoku),
            CellOptions::from(&[7, 9])
        );
        solver.advance(&mut state);
        assert_eq!(*state.sudoku.cell(Cell::new(0, 5)), 7);
        solver.advance(&mut state);
        assert_eq!(*state.sudoku.cell(Cell::new(0, 5)), 9);
        assert!(matches!(solver.advance(&mut state), AdvanceResult::Invalid));
    }
}
