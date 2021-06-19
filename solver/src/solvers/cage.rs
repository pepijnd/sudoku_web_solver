use crate::{Cell, CellMod, CellOptions, EntrySolver, State, StateMod, util::Domain};

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
        println!("cage data collecting");
        for (cage, &total) in cages.cages.iter().enumerate() {
            let mut unsolved = false;
            let mut size = 0;
            let cage = cage as u32 + 1;
            let mut options = CellOptions::default();
            let mut digits = CellOptions::default();
            for (index, &cell_cage) in cages.cells.iter().enumerate() {
                if cell_cage != cage { continue }
                let cell = Cell::from_index(index);
                options.combine(&state.options.options(cell, &state.sudoku));
                let digit = *state.sudoku.cell(cell);
                if digit == 0 {
                    unsolved = true;
                }
                digits.add(digit);
                size += 1;
            }
            if options.len() < size as usize {
                return false
            }
            if unsolved {
                size_options.push((size, total, options, digits));
            }
        }
        for (cage, &(size, total, options, digits)) in size_options.iter().enumerate() {
            let mut mods = StateMod::from(state.info.tech);
            let cage = cage as u32 + 1;
            let mut sums = Self::sums(size, total);
            sums.retain(|sum| options.is_set(sum) && sum.is_set(&digits));
            if let Some(options) = sums.iter_mut().reduce(|a, b| { a.combine(b); a }) {
                for (index, &cell_cage) in cages.cells.iter().enumerate() {
                    if cell_cage != cage { continue }
                    let cell = Cell::from_index(index);
                    let cell_options = state.options.options(cell, &state.sudoku);
                    for cell_option in cell_options.iter() {
                        if !options.has(cell_option) {
                            state.remove(cell, cell_option);
                            mods.push_target(CellMod::option(cell, cell_option))
                        }
                    }
                }                
            } else {
                return false
            }
            if mods.has_targets() {
                state.info.push_mod(mods);
            }
        }
        true
    }

    pub fn sums(size: u32, total: u32) -> Vec<CellOptions> {
        if size == 1 {
            let mut option = CellOptions::default();
            option.add(total as u8);
            return vec![option]
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
                    (1..=9)
                        .filter_map(move |x| {
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

    fn subset(size: usize, mut total: usize) {}
}

impl Default for CageSolver {
    fn default() -> Self {
        Self
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = CageSolver::sums(1, 9);
        let l = list.len();
        for o in list {
            println!("{}: {:?}", o.sum(), o.iter().collect::<Vec<_>>());
        }
        println!("len: {}", l);
    }
}
