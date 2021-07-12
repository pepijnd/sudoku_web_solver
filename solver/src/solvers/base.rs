use crate::config::Config;
use crate::solving::{CellMod, Entry, Reporter, StateMod};
use crate::{AdvanceResult, Cell, EntrySolver, Options, Solver, State, Sudoku};

#[derive(Debug, Copy, Clone)]
pub struct StateInit;

impl EntrySolver for StateInit {
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
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
    fn advance(_state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
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
    fn advance(_state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        AdvanceResult::Invalid
    }

    fn terminate() -> bool {
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
    fn advance(_state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        AdvanceResult::Invalid
    }

    fn terminate() -> bool {
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
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        state.info.entry.correct = false;
        AdvanceResult::Invalid
    }

    fn terminate() -> bool {
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
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        let mut solved = true;
        let mut mods = StateMod::from(state.info.entry.tech);
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
        state.info.entry.solved = solved;
        AdvanceResult::Advance
    }
}

impl Default for BaseSolver {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Backtrace;

impl EntrySolver for Backtrace {
    fn advance(state: &mut State, config: &Config, reporter: &mut Reporter) -> AdvanceResult {
        state.info.entry.correct = false;
        let State {
            info,
            sudoku,
            options,
            caches,
        } = state;

        if info.backtrace().job {
            return AdvanceResult::Advance;
        }
        if info.backtrace().cell.is_none() {
            if let Some(cell) = Self::heuristic(sudoku, options, config) {
                info.backtrace().cell.replace(cell);
                info.backtrace().options = options.options(cell, sudoku);
                info.entry.splits *= info.backtrace().options.len() as u32;
            } else {
                return AdvanceResult::Invalid;
            }
        }
        if let Some(max_splits) = config.max_splits {
            let cell = info
                .backtrace()
                .cell
                .expect("target cell should be set at this point");
            if info.entry.splits < max_splits.get() {
                let mut jobs = Vec::new();
                let cell_options = info.backtrace().options;
                for value in cell_options.iter() {
                    let mods = StateMod::from_change(info.entry.tech, cell, value);
                    let mut state = State {
                        sudoku: *sudoku,
                        options: *options,
                        info: info.clone(),
                        caches: caches.clone(),
                    };
                    state.info.push_mod(mods);
                    state.update(cell, value);
                    state.info.entry.depth += 1;
                    jobs.push(Entry {
                        state,
                        solver: Solver::BackTrace,
                    })
                }
                return AdvanceResult::Split(jobs);
            }
        }

        reporter.progress(info.backtrace().retries, info.entry.splits);

        if let Some(value) = info.backtrace().options.take() {
            info.backtrace().retries += 1;
            let cell = info
                .backtrace()
                .cell
                .expect("target cell should be set at this point");
            state.update(cell, value);
            let mods = StateMod::from_change(state.info.entry.tech, cell, value);
            state.info.push_mod(mods);
            AdvanceResult::Advance
        } else {
            AdvanceResult::Invalid
        }
    }

    fn verified(state: &State) -> bool {
        if let Some(ref info) = state.info.backtrace {
            info.job
        } else {
            false
        }
    }
}

impl Backtrace {
    pub fn heuristic(sudoku: &Sudoku, options: &mut Options, config: &Config) -> Option<Cell> {
        let mut candidate: Option<(usize, Cell)> = None;
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell::new(row, col);
                if *sudoku.cell(cell) != 0 {
                    continue;
                };
                let options = options.options(cell, sudoku);
                let mut score = 10 - options.len();
                if config.rules.cages.cells[cell.index()] != 0 {
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
        Self {}
    }
}

#[cfg(test)]
mod test {
    use super::Backtrace;
    use crate::{
        config::Config, solving::Reporter, AdvanceResult, Cell, CellOptions, EntrySolver, State,
        Sudoku,
    };

    static SAMPLE: &str =
        "...6..8....35.4...65..217...6..............5..7138..2...7.1.6.4.1.......9....3..7";

    #[test]
    fn backtrace_test() {
        let sudoku = Sudoku::from(SAMPLE);
        let mut state = State {
            sudoku,
            ..Default::default()
        };
        assert_eq!(
            state.options.options(Cell::new(0, 5), &state.sudoku),
            CellOptions::from(&[7, 9])
        );
        let mut reporter = Reporter::default();
        let config = Config::default();
        Backtrace::advance(&mut state, &config, &mut reporter);
        assert_eq!(*state.sudoku.cell(Cell::new(0, 5)), 7);
        Backtrace::advance(&mut state, &config, &mut reporter);
        assert_eq!(*state.sudoku.cell(Cell::new(0, 5)), 9);
        assert!(matches!(
            Backtrace::advance(&mut state, &config, &mut reporter),
            AdvanceResult::Invalid
        ));
    }
}
