use crate::{util::Domain, Cell, CellMod, EntrySolver, ModMarking, State, StateMod};

#[derive(Debug, Copy, Clone)]
pub struct SingleSolver;

impl EntrySolver for SingleSolver {
    fn advance(&mut self, state: &mut State) -> bool {
        for d in 0..9 {
            Self::test(Domain::Row(d), state);
            Self::test(Domain::Col(d), state);
            Self::test(Domain::Sqr(d), state);
        }
        true
    }
}

impl SingleSolver {
    fn test(domain: Domain, state: &mut State) {
        'value: for value in 1..=9 {
            let mut mods = StateMod::from(state.info.tech);
            mods.push_mark(ModMarking::Domain(domain));
            let mut found: Option<Cell> = None;
            for i in 0..9 {
                let cell = domain.cell(i);
                if *state.sudoku.cell(cell) == value {
                    continue 'value;
                }
                if *state.sudoku.cell(cell) != 0 {
                    continue;
                }
                if state.options.options(cell, &state.sudoku).has(value) {
                    if found.is_some() {
                        continue 'value;
                    } else {
                        found = Some(cell);
                    }
                }
            }
            if let Some(cell) = found {
                state.update(cell, value);
                mods.push_target(CellMod::digit(cell, value));
            }
            if mods.has_targets() {
                state.info.push_mod(mods);
            }
        }
    }
}

impl Default for SingleSolver {
    fn default() -> Self {
        Self
    }
}
