use crate::solving::{CellMod, StateMod};
use crate::util::SetDomain;
use crate::{AdvanceResult, EntrySolver, State};

#[derive(Debug, Copy, Clone)]
pub struct RowSet {
    row: usize,
    first: Option<usize>,
    second: Option<usize>,
}

impl RowSet {
    fn new(row: usize) -> Self {
        RowSet {
            row,
            first: None,
            second: None,
        }
    }

    fn add(&mut self, i: usize) -> bool {
        if self.first.is_none() {
            self.first = Some(i);
        } else if self.second.is_none() {
            self.second = Some(i);
        } else {
            return false;
        }
        true
    }

    fn valid(&self) -> bool {
        self.first.is_some() && self.second.is_some()
    }

    fn same(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XWingSolver;

impl EntrySolver for XWingSolver {
    fn advance(&mut self, state: &mut State) -> AdvanceResult {
        for nr in 1..=9 {
            Self::test(SetDomain::Row, nr, state);
            Self::test(SetDomain::Col, nr, state);
        }
        AdvanceResult::Advance
    }
}

impl XWingSolver {
    fn test(d: SetDomain, nr: u8, state: &mut State) {
        let mut rows = Vec::new();
        'n: for n in 0..9 {
            let mut row = RowSet::new(n);
            for i in 0..9 {
                let cell = d.cell(n, i);
                let options = state.options.options(cell, &state.sudoku);
                if options.has(nr) && !row.add(i) {
                    continue 'n;
                }
            }
            if row.valid() {
                rows.push(row);
            }
        }
        if rows.len() > 1 {
            for i in 0..rows.len() {
                for j in 0..rows.len() {
                    if i == j {
                        continue;
                    }
                    if rows[i].same(&rows[j]) {
                        Self::xwing(d, nr, rows[i], rows[j], state);
                    }
                }
            }
        }
    }

    fn xwing(d: SetDomain, value: u8, first: RowSet, second: RowSet, state: &mut State) {
        let d = d.other();
        let mut mods = StateMod::from(state.info.tech);
        for i in 0..9 {
            if i == first.row || i == second.row {
                continue;
            }

            let cell = d.cell(first.first.unwrap(), i);
            if state.remove(cell, value) {
                mods.push_target(CellMod::option(cell, value));
            }

            let cell = d.cell(first.second.unwrap(), i);
            if state.remove(cell, value) {
                mods.push_target(CellMod::option(cell, value));
            }
        }
        if mods.has_targets() {
            mods.push_source(d.cell(first.first.unwrap(), first.row).into());
            mods.push_source(d.cell(first.second.unwrap(), first.row).into());
            mods.push_source(d.cell(second.first.unwrap(), second.row).into());
            mods.push_source(d.cell(second.second.unwrap(), second.row).into());
            state.info.push_mod(mods);
        }
    }
}

impl Default for XWingSolver {
    fn default() -> Self {
        Self
    }
}
