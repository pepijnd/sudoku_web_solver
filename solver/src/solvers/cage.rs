use crate::{util::Domain, CellOptions, EntrySolver, State};

use arrayvec::ArrayVec;

#[derive(Debug, Copy, Clone)]
pub struct CageSolver;

impl EntrySolver for CageSolver {
    fn advance(&mut self, state: &mut State) -> bool {
        false
    }
}

impl CageSolver {
    fn test(domain: Domain, state: &mut State) {}

    pub fn sums(size: usize, total: usize) -> Vec<CellOptions> {
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
        let list = CageSolver::sums(4, 26);
        let l = list.len();
        for o in list {
            println!("{}: {:?}", o.sum(), o.iter().collect::<Vec<_>>());
        }
        println!("len: {}", l);
    }
}
