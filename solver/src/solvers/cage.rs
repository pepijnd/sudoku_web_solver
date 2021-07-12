use smallvec::SmallVec;

use crate::config::Config;
use crate::solving::{CellMod, Reporter, StateMod};
use crate::{AdvanceResult, Cell, CellOptions, EntrySolver, State};

#[derive(Debug, Copy, Clone)]
pub struct CageSolver;

impl EntrySolver for CageSolver {
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        Self::test(state, config)
    }
}

#[derive(Debug)]
enum CellState {
    Digit(u8),
    Option(CellOptions),
}

impl CageSolver {
    fn test(state: &mut State, config: &Config) -> AdvanceResult {
        let cages = config.rules.cages.clone();

        for (cage, &total) in cages.cages.iter().enumerate() {
            let mut cage_cells: SmallVec<[(Cell, CellState); 9]> = SmallVec::new();
            for (index, &cell_cage) in cages.cells.iter().enumerate() {
                if cell_cage != cage + 1 {
                    continue;
                }
                let cell = Cell::from_index(index);
                let value = *state.sudoku.cell(cell);
                if value != 0 {
                    cage_cells.push((cell, CellState::Digit(value)));
                } else {
                    let options = state.cell_options(cell);
                    cage_cells.push((cell, CellState::Option(options)));
                }
            }
            let size = cage_cells.len();
            let mut sums = (0..size)
                .map(|i| (cage_cells[i].0, CellOptions::default()))
                .collect::<SmallVec<[_; 9]>>();
            let mut buffer = (0..size).map(|_| (0, 0)).collect::<SmallVec<[_; 9]>>();
            let mut i = 0;
            let mut valid = true;
            let mut test = false;
            loop {
                let (_, state) = &cage_cells[i];
                match state {
                    CellState::Digit(digit) => {
                        if valid && !buffer.iter().any(|(_, v)| *v == *digit as u32) {
                            buffer[i].1 = *digit as u32;
                        } else {
                            buffer[i].1 = 0;
                            valid = false;
                        }
                    }
                    CellState::Option(options) => {
                        if let Some(option) = options.iter().nth(buffer[i].0) {
                            buffer[i].0 += 1;
                            valid = true;
                            if !buffer.iter().any(|(_, v)| *v == option as u32) {
                                buffer[i].1 = option as u32;
                            } else {
                                buffer[i].1 = 0;
                                i -= 1;
                            }
                        } else {
                            buffer[i] = (0, 0);
                            valid = false;
                        }
                    }
                }
                if valid {
                    i += 1;
                } else {
                    if i == 0 {
                        if !test {
                            return AdvanceResult::Invalid;
                        }
                        break;
                    }
                    i -= 1;
                }
                if i == size && valid {
                    let sum = buffer.iter().map(|(_, v)| *v).sum::<u32>();
                    if sum == total {
                        for ((_, options), (_, digit)) in sums.iter_mut().zip(buffer.iter()) {
                            options.add(*digit as u8);
                        }
                        test = true;
                    }
                    i -= 1;
                    valid = false;
                }
            }
            let mut mods = StateMod::from(state.info.entry.tech);
            for &(cell, options) in &sums {
                for i in 1..=9 {
                    if !options.has(i) && state.remove_option(cell, i) {
                        mods.push_target(CellMod::option(cell, i));
                    }
                }
            }
            if mods.has_targets() {
                state.info.push_mod(mods);
            }
        }
        AdvanceResult::Advance
    }
}

impl Default for CageSolver {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::CageSolver;
    use crate::config::Config;
    use crate::rules::{Cages, Rules};
    use crate::solving::Reporter;
    use crate::{AdvanceResult, EntrySolver, State, Sudoku};

    #[test]
    fn test() {
        let mut config = Config {
            rules: Rules {
                cages: Cages {
                    cages: vec![20, 27, 26, 24, 28, 17, 18, 30, 16, 24],
                    cells: [
                        0, 0, 0, 0, 1, 2, 2, 2, 3, 0, 0, 0, 0, 1, 1, 1, 2, 3, 0, 0, 0, 0, 4, 4, 5,
                        5, 3, 0, 0, 0, 0, 0, 4, 4, 5, 3, 6, 7, 8, 0, 0, 0, 4, 5, 3, 6, 7, 8, 8, 0,
                        0, 0, 0, 0, 6, 7, 7, 8, 8, 0, 0, 0, 0, 6, 9, 10, 10, 10, 0, 0, 0, 0, 6, 9,
                        9, 9, 10, 0, 0, 0, 0,
                    ],
                },
            },
            ..Default::default()
        };
        config.add_rules_solvers();
        let mut state = State {
            sudoku: Sudoku::from(
                ".....8...........................................................................",
            ),
            ..Default::default()
        };
        let mut reporter = Reporter::default();
        assert!(matches!(
            CageSolver::advance(&mut state, &config, &mut reporter),
            AdvanceResult::Advance
        ));
    }
}
