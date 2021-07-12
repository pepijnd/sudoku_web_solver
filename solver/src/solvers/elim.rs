use crate::config::Config;
use crate::solving::{CellMod, Reporter, StateMod};
use crate::util::SetDomain;
use crate::{AdvanceResult, Cell, EntrySolver, State};

#[derive(Debug, Copy, Clone)]
pub struct ElimSolver;

impl EntrySolver for ElimSolver {
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        for value in 1..=9 {
            for i in 0..9 {
                Self::test_sqr(i, value, state);
                Self::test(i, SetDomain::Row, value, state);
                Self::test(i, SetDomain::Col, value, state);
            }
        }
        AdvanceResult::Advance
    }
}

impl ElimSolver {
    fn test_sqr(sqr: usize, value: u8, state: &mut State) {
        let mut row = None;
        let mut col = None;
        let mut mods = StateMod::from(state.info.entry.tech);

        for i in 0..9 {
            let cell = Cell::from_sqr(sqr, i);
            if *state.sudoku.cell(cell) == value {
                return;
            }
            if *state.sudoku.cell(cell) != 0 {
                continue;
            }
            let options = state.cell_options(cell);
            if options.has(value) {
                if !mods.has_source() {
                    row = Some(cell.row);
                    col = Some(cell.col);
                } else {
                    if let Some(r) = row {
                        if r != cell.row {
                            row = None;
                        }
                    }
                    if let Some(c) = col {
                        if c != cell.col {
                            col = None;
                        }
                    }
                }
                mods.push_source(CellMod::option(cell, value));
            }
        }

        if mods.num_sources() < 2 {
            return;
        }

        if let Some(row) = row {
            for i in 0..9 {
                let cell = Cell::new(row, i);
                if cell.sqr() == sqr || *state.sudoku.cell(cell) != 0 {
                    continue;
                }
                if state.remove_option(cell, value) {
                    mods.push_target(CellMod::option(cell, value));
                }
            }
        } else if let Some(col) = col {
            for i in 0..9 {
                let cell = Cell::new(i, col);
                if cell.sqr() == sqr || *state.sudoku.cell(cell) != 0 {
                    continue;
                }
                if state.remove_option(cell, value) {
                    mods.push_target(CellMod::option(cell, value));
                }
            }
        }

        if mods.has_targets() {
            state.info.push_mod(mods);
        }
    }

    fn test(n: usize, d: SetDomain, value: u8, state: &mut State) {
        let mut sqr = None;
        let mut mods = StateMod::from(state.info.entry.tech);

        for i in 0..9 {
            let cell = d.cell(n, i);
            if *state.sudoku.cell(cell) == value {
                return;
            }
            if *state.sudoku.cell(cell) != 0 {
                continue;
            }
            let options = state.cell_options(cell);
            if options.has(value) {
                if sqr.is_none() {
                    sqr = Some(cell.sqr());
                } else if let Some(sqr) = sqr {
                    if sqr != cell.sqr() {
                        return;
                    }
                }
                mods.push_source(CellMod::option(cell, value));
            }
        }

        if mods.num_sources() < 2 {
            return;
        }

        if let Some(sqr) = sqr {
            for i in 0..9 {
                let cell = Cell::from_sqr(sqr, i);
                if d.is(cell, n) || *state.sudoku.cell(cell) != 0 {
                    continue;
                }
                if state.remove_option(cell, value) {
                    mods.push_target(CellMod::option(cell, value));
                }
            }
        }

        if mods.has_targets() {
            state.info.push_mod(mods)
        }
    }
}

impl Default for ElimSolver {
    fn default() -> Self {
        Self
    }
}
