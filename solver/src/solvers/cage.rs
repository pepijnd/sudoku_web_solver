use crate::{Cell, CellMod, CellOptions, EntrySolver, State, StateMod};

#[derive(Debug, Copy, Clone)]
pub struct CageSolver;

impl EntrySolver for CageSolver {
    fn advance(&mut self, state: &mut State) -> bool {
        Self::test(state)
    }
}

impl CageSolver {
    fn test(state: &mut State) -> bool {
        let cages = state.config.rules.cages.clone();
        let mut size_options = Vec::with_capacity(cages.cages.len());

        for (cage, &total) in cages.cages.iter().enumerate() {
            let mut unsolved = false;
            let mut size = 0;
            let cage = cage as u32 + 1;
            let mut options = CellOptions::default();
            let mut digits = CellOptions::default();
            let mut sum = 0;
            for (index, &cell_cage) in cages.cells.iter().enumerate() {
                if cell_cage != cage {
                    continue;
                }
                let cell = Cell::from_index(index);
                options.combine(&state.options.options(cell, &state.sudoku));
                let digit = *state.sudoku.cell(cell);
                if digit == 0 {
                    unsolved = true;
                } else {
                    digits.add(digit);
                    sum += digit as u32;
                }
                size += 1;
            }
            if options.len() < size as usize {
                return false;
            }
            if unsolved {
                size_options.push((cage, size, total, options, digits));
            } else if sum != total {
                return false
            }
        }
        for &(cage, size, total, options, digits) in &size_options{
            let mut mods = StateMod::from(state.info.tech);
            let mut sums = Self::sums(size, total);
            sums.retain(|sum| options.is_set(sum) && sum.is_set(&digits));
            if let Some(options) = sums.iter_mut().reduce(|a, b| {
                a.combine(b);
                a
            }) {
                for (index, &cell_cage) in cages.cells.iter().enumerate() {
                    if cell_cage != cage {
                        continue;
                    }
                    let cell = Cell::from_index(index);
                    if *state.sudoku.cell(cell) != 0 {
                        continue;
                    }
                    let cell_options = state.options.options(cell, &state.sudoku);
                    for digit in digits.iter() {
                        if state.remove(cell, digit) {
                            mods.push_target(CellMod::option(cell, digit))
                        }
                    }
                    for cell_option in cell_options.iter() {
                        if !options.has(cell_option) && state.remove(cell, cell_option) {
                            mods.push_target(CellMod::option(cell, cell_option))
                        }
                    }
                }
            } else {
                return false;
            }
            if mods.has_targets() {
                state.info.push_mod(mods);
            }
        }
        true
    }

    pub fn sums(size: u32, total: u32) -> Vec<CellOptions> {
        if size == 1 && total <= 9 {
            let mut option = CellOptions::default();
            option.add(total as u8);
            return vec![option];
        }
        let mut options: Vec<CellOptions> = (1..=9)
            .map(|x| {
                let mut a = CellOptions::default();
                a.add(x);
                a
            })
            .collect();
        for n in 1..size {
            options = options
                .iter()
                .flat_map(move |&o| {
                    (1..=9).filter_map(move |x| {
                        let mut l = o;
                        let sum = o.sum() + x;
                        if l.has(x as u8) || sum > total || (size - 1 == n && sum != total) {
                            None
                        } else {
                            l.add(x as u8);
                            Some(l)
                        }
                    })
                })
                .collect();
            options.sort_unstable();
            options.dedup();
        }
        options
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

    #[test]
    fn test() {
        let list = CageSolver::sums(5, 26);
        let l = list.len();
        for o in list {
            println!("{}: {:?}", o.sum(), o.iter().collect::<Vec<_>>());
        }
        println!("len: {}", l);
    }
}
