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

#[derive(Debug, Clone, Copy)]
enum Found {
    None,
    Single(Cell),
    More,
}

impl SingleSolver {
    fn test(domain: Domain, state: &mut State) {
        let mut mods = StateMod::from(state.info.tech);
        mods.push_mark(ModMarking::Domain(domain));

        let options = (0..9)
            .map(|i| domain.cell(i))
            .filter_map(|cell| {
                if *state.sudoku.cell(cell) == 0 {
                    Some((cell, state.options.options(cell, &state.sudoku)))
                } else {
                    None
                }
            })
            .fold([Found::None; 9], |mut a, (cell, options)| {
                options.iter().for_each(|o| {
                    debug_assert!(o <= 9);
                    debug_assert!(o > 0);
                    let i = (o - 1) as usize;
                    a[i] = match a[i] {
                        Found::None => Found::Single(cell),
                        Found::Single(_) => Found::More,
                        Found::More => Found::More,
                    }
                });
                a
            });

        for (index, count) in options.iter().enumerate() {
            let value = (index + 1) as u8;
            if let Found::Single(cell) = count {
                state.update(*cell, value);
                mods.push_target(CellMod::digit(*cell, value));
            }
        }

        if mods.has_targets() {
            state.info.push_mod(mods);
        }
    }
}

impl Default for SingleSolver {
    fn default() -> Self {
        Self
    }
}
