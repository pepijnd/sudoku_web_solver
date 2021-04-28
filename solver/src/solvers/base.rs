use crate::{Cell, CellMod, CellOptions, EntrySolver, State, StateMod};

#[derive(Debug, Copy, Clone)]
pub struct StateInit;

impl EntrySolver for StateInit {
    fn advance(&mut self, state: &mut State) -> bool {
        state.info.push_state();
        true
    }
}

impl Default for StateInit {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateSolved;

impl EntrySolver for StateSolved {
    fn advance(&mut self, _state: &mut State) -> bool {
        false
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
    fn advance(&mut self, _state: &mut State) -> bool {
        false
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
pub struct BaseSolver;

impl EntrySolver for BaseSolver {
    fn advance(&mut self, state: &mut State) -> bool {
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
                        return false;
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
        true
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
}

impl EntrySolver for Backtrace {
    fn advance(&mut self, state: &mut State) -> bool {
        state.info.correct = false;
        if let Some((cell, value)) = self.next(state) {
            state.info.guesses += 1;
            state.info.guesses_t += 1;
            let mut mods = StateMod::from(state.info.tech);
            mods.push_target(cell.into());
            state.info.push_mod(mods);
            state.update(cell, value);
            true
        } else {
            false
        }
    }

    fn verified(&self) -> bool {
        false
    }
}

impl Backtrace {
    pub fn next(&mut self, state: &mut State) -> Option<(Cell, u8)> {
        if let Some(cell) = self.cell {
            if let Some(value) = self.options.take() {
                return Some((cell, value));
            }
            None
        } else {
            let mut candidate: Option<(usize, Cell, CellOptions)> = None;
            'lowest: for row in 0..9 {
                for col in 0..9 {
                    let cell = Cell { row, col };
                    if *state.sudoku.cell(cell) != 0 {
                        continue;
                    };
                    let options = state.options.options(cell, &state.sudoku);
                    let len = options.len();
                    if candidate.is_none() || len < candidate.unwrap().0 {
                        candidate.replace((len, Cell { row, col }, options));
                        if len == 2 {
                            break 'lowest;
                        }
                    }
                }
            }
            if let Some((_, cell, options)) = candidate {
                self.cell = Some(cell);
                self.options = options;
                if let Some(option) = self.options.take() {
                    return Some((cell, option));
                }
            }
            None
        }
    }
}

impl Default for Backtrace {
    fn default() -> Self {
        Self {
            cell: None,
            options: CellOptions::all(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Cell, CellOptions, EntrySolver, State, Sudoku};

    use super::Backtrace;

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
    }
}
