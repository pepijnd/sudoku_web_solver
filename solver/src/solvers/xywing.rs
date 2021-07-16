use crate::config::Config;
use crate::options::OptionPair;
use crate::solving::{CellMod, Reporter, StateMod};
use crate::util::SetDomain;
use crate::{AdvanceResult, Cell, CellOptions, EntrySolver, State};

#[derive(Debug, Copy, Clone)]
pub struct XYWingSolver;

impl EntrySolver for XYWingSolver {
    fn advance(state: &mut State, config: &Config, _reporter: &mut Reporter) -> AdvanceResult {
        for row in 0..9 {
            for col in 0..9 {
                let cell = Cell::new(row, col);
                let c_opts = state.cell_options(cell);
                if *state.sudoku.cell(cell) != 0 {
                    continue;
                }
                if let Some(c_pair) = c_opts.as_pair() {
                    Self::test_cell(cell, c_opts, c_pair, state);
                }
            }
        }
        AdvanceResult::Advance
    }
}

impl XYWingSolver {
    fn test_cell(cell: Cell, c_opts: CellOptions, c_pair: OptionPair, state: &mut State) {
        let mut matches: smallvec::SmallVec<[(OptionPair, Cell); 6]> = smallvec::SmallVec::new();
        for matching in (0..9)
            .map(|i| {
                [SetDomain::Row, SetDomain::Col, SetDomain::Sqr]
                    .iter()
                    .map(move |d| d.matching(cell, i))
            })
            .flatten()
        {
            let m_opts = state.cell_options(matching);
            if matching != cell
                && *state.sudoku.cell(matching) == 0
                && c_opts != m_opts
                && !matches.iter().any(|(_, m)| *m == matching)
            {
                if let Some(m_pair) = m_opts.as_pair() {
                    if let Some(common) = c_pair.common(m_pair) {
                        for (value, other, _o_pair) in matches
                            .iter()
                            .filter(|&(c, _)| *c != m_pair)
                            .filter_map(|&(c, m)| c.common(m_pair).map(|x| (x, m, c)))
                        {
                            Self::test_other(cell, matching, other, value, common, state);
                        }
                        matches.push((m_pair, matching));
                    }
                }
            }
        }
    }

    fn test_other(
        cell: Cell,
        matching: Cell,
        other: Cell,
        value: u8,
        common: u8,
        state: &mut State,
    ) {
        if matching.sees(other) || value == common {
            return;
        }
        let mut mods = StateMod::from(state.info.entry.tech);
        for d in &[SetDomain::Row, SetDomain::Col, SetDomain::Sqr] {
            for i in 0..9 {
                let elim = d.matching(matching, i);
                if *state.sudoku.cell(elim) != 0 { continue }
                if other.sees(elim)
                    && elim != other
                    && elim != matching
                    && elim != cell
                    && state.remove_option(elim, value)
                {
                    mods.push_target(CellMod::option(elim, value));
                }
            }
        }
        if mods.has_targets() {
            mods.push_source(cell.into());
            mods.push_source(matching.into());
            mods.push_source(other.into());
            state.info.push_mod(mods);
        }
    }
}

impl Default for XYWingSolver {
    fn default() -> Self {
        Self
    }
}
