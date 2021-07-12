use crate::config::Config;
use crate::solving::{CellMod, Reporter, StateMod};
use crate::util::SetDomain;
use crate::{AdvanceResult, Cell, EntrySolver, State};

#[derive(Debug, Copy, Clone)]
pub struct SetSolver;

impl EntrySolver for SetSolver {
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        for domain in 0..9 {
            for cell in 0..9 {
                Self::test(domain, cell, SetDomain::Sqr, state);
                Self::test(domain, cell, SetDomain::Row, state);
                Self::test(domain, cell, SetDomain::Col, state);
            }
        }
        AdvanceResult::Advance
    }
}

impl SetSolver {
    fn test(d: usize, i: usize, domain: SetDomain, state: &mut State) {
        let cell = domain.cell(d, i);
        if *state.sudoku.cell(cell) != 0 {
            return;
        }
        let options = state.options.options(cell, &state.sudoku);
        let len = options.len();
        let mut set = smallvec::SmallVec::<[Cell; 6]>::new();
        set.push(cell);
        for c in 0..9 {
            let cmp = domain.cell(d, c);
            if cmp == cell || *state.sudoku.cell(cmp) != 0 {
                continue;
            }
            let other = state.options.options(cmp, &state.sudoku);
            if options.is_set(&other) {
                set.push(cmp);
            }
        }
        if len != set.len() || len < 2 {
            return;
        }
        let mut mods = StateMod::from(state.info.entry.tech);
        for other in 0..9 {
            let other = domain.cell(d, other);
            if set.contains(&other) || *state.sudoku.cell(other) != 0 {
                continue;
            }
            for value in options.iter() {
                if state.remove(other, value) {
                    mods.push_target(CellMod::option(other, value));
                }
            }
        }

        if len <= 3 && domain == SetDomain::Sqr {
            let mut row = Some(cell.row);
            let mut col = Some(cell.col);
            for other in &set {
                if let Some(r) = row {
                    if r != other.row {
                        row = None;
                    }
                }
                if let Some(c) = col {
                    if c != other.col {
                        col = None;
                    }
                }
            }
            if let Some(row) = row {
                for col in 0..9 {
                    let other = Cell::new(row, col);
                    if set.contains(&other) || *state.sudoku.cell(other) != 0 {
                        continue;
                    }
                    for value in options.iter() {
                        if state.remove(other, value) {
                            mods.push_target(CellMod::option(other, value));
                        }
                    }
                }
            }
            if let Some(col) = col {
                for row in 0..9 {
                    let other = Cell::new(row, col);
                    if set.contains(&other) || *state.sudoku.cell(other) != 0 {
                        continue;
                    }
                    for value in options.iter() {
                        if state.remove(other, value) {
                            mods.push_target(CellMod::option(other, value));
                        }
                    }
                }
            }
        }
        for source in set {
            mods.push_source(source.into());
        }
        if mods.has_targets() {
            state.info.push_mod(mods);
        }
    }
}

impl Default for SetSolver {
    fn default() -> Self {
        Self
    }
}
