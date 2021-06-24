use std::{io::Write, time::UNIX_EPOCH};

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
pub struct StateInvalid;

impl EntrySolver for StateInvalid {
    fn advance(&mut self, state: &mut State) -> bool {
        state.info.correct = false;
        false
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
        let advance = if let Some((cell, value, size)) = self.next(state) {
            if state.info.total.is_none() {
                state.info.total = Some(size);
                state.info.progress.push((0, size));
            } else {
                state.info.progress.last_mut().unwrap().0 = state.info.guesses;
            }
            state.info.guesses += 1;
            state.info.guesses_t += 1;
            let mut mods = StateMod::from(state.info.tech);
            mods.push_target(cell.into());
            state.info.push_mod(mods);
            state.update(cell, value);

            true
        } else {
            false
        };

        let mut chance = 0.0;
        let mut part = 1;
        for &(g, t) in &state.info.progress {
            chance += (g as f64 / t as f64) / part as f64;
            part *= t;
        }
        advance
    }

    fn verified(&self) -> bool {
        false
    }
}

impl Backtrace {
    pub fn next(&mut self, state: &mut State) -> Option<(Cell, u8, u32)> {
        if let Some(cell) = self.cell {
            let size = self.options.len();
            if let Some(value) = self.options.take() {
                return Some((cell, value, size as u32));
            }
            None
        } else {
            let mut candidate: Option<(usize, Cell, CellOptions)> = None;
            for row in 0..9 {
                for col in 0..9 {
                    let cell = Cell { row, col };
                    if *state.sudoku.cell(cell) != 0 {
                        continue;
                    };
                    let options = state.options.options(cell, &state.sudoku);
                    let cell = Cell::new(row, col);
                    let mut score = 10-options.len();
                    if state.config.rules.cages.cells[cell.index()] != 0 {
                        score *= 2;
                    }
                    if candidate.is_none() || score > candidate.unwrap().0 {
                        candidate.replace((score, cell, options));
                        
                    }
                }
            }
            if let Some((_, cell, options)) = candidate {
                self.cell = Some(cell);
                self.options = options;
                let size = self.options.len();
                if let Some(option) = self.options.take() {
                    return Some((cell, option, size as u32));
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
