pub mod options;
pub mod output;
pub mod solvers;
pub mod sudoku;
pub mod util;

use std::rc::Rc;

use serde::{Deserialize, Serialize};
use util::Domain;

#[doc(inline)]
pub use {
    options::{CellOptions, Options},
    output::{Solve, SolveStep},
    solvers::Solver,
    sudoku::Sudoku,
    util::Cell,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub state: State,
    pub solver: Solver,
    pub entry: Box<dyn EntrySolver>,
}

impl Entry {
    pub fn new(sudoku: Sudoku, options: Options, tech: Solver, config: Rc<Config>) -> Self {
        Self {
            state: State {
                sudoku,
                options,
                info: Info::default(),
                config,
            },
            solver: tech,
            entry: tech.make(),
        }
    }

    pub fn from_state(state: State) -> Self {
        let entry = state.info.tech.make();
        let tech = state.info.tech;
        Self {
            state,
            solver: tech,
            entry,
        }
    }

    pub fn make_next(&self) -> Entry {
        let mut state = self.state.clone();
        state.info.reset();

        if self.info.solved {
            state.info.tech = Solver::Solved;
            state.info.push_state();
            return Entry::from_state(state);
        }

        if self.info.change {
            state.info.tech = self.config.base;
            return Entry::from_state(state);
        }

        let mut next = false;
        for &tech in self.config.solvers.iter() {
            if self.solver == self.config.base || next {
                state.info.tech = tech;
                return Entry::from_state(state);
            } else if tech == self.solver {
                next = true;
            }
        }

        if let Some(tech) = self.config.fallback {
            state.info.tech = tech;
            Entry::from_state(state)
        } else {
            state.info.tech = Solver::Incomplete;
            state.info.push_state();
            Entry::from_state(state)
        }
    }

    pub fn advance(&mut self) -> bool {
        self.state.info.tech = self.solver;
        self.entry.advance(&mut self.state)
    }

    pub fn verified(&self) -> bool {
        self.entry.verified()
    }

    pub fn terminate(&self) -> bool {
        self.entry.terminate()
    }
}

impl std::ops::Deref for Entry {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl std::ops::DerefMut for Entry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CellMod {
    cell: Cell,
    target: ModTarget,
}

impl CellMod {
    fn digit(cell: Cell, digit: u8) -> CellMod {
        CellMod {
            cell,
            target: ModTarget::Digit(digit),
        }
    }

    fn option(cell: Cell, option: u8) -> CellMod {
        CellMod {
            cell,
            target: ModTarget::Option(option),
        }
    }

    fn apply(&self, s: &mut Sudoku, c: &mut Options) -> bool {
        match self.target {
            ModTarget::Digit(n) => {
                s.set_cell(self.cell, n);
                true
            }
            ModTarget::Option(n) => c.remove(self.cell, n),
            ModTarget::Cell => false,
        }
    }
}

impl From<Cell> for CellMod {
    fn from(cell: Cell) -> Self {
        Self {
            cell,
            target: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ModMarking {
    Domain(Domain),
    Cell(Cell),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateMod {
    solver: Solver,
    source: smallvec::SmallVec<[CellMod; 6]>,
    target: smallvec::SmallVec<[CellMod; 6]>,
    marks: smallvec::SmallVec<[ModMarking; 4]>,
}

impl StateMod {
    pub fn push_source(&mut self, source: CellMod) {
        self.source.push(source);
    }

    pub fn has_source(&self) -> bool {
        !self.source.is_empty()
    }

    pub fn num_sources(&self) -> usize {
        self.source.len()
    }

    pub fn push_target(&mut self, target: CellMod) {
        self.target.push(target);
    }

    pub fn has_targets(&self) -> bool {
        !self.target.is_empty()
    }

    pub fn push_mark(&mut self, mark: ModMarking) {
        self.marks.push(mark);
    }

    pub fn is_target(&self, cell: Cell) -> bool {
        self.target.iter().any(|c| c.cell == cell)
    }

    pub fn is_target_option(&self, cell: Cell, option: u8) -> bool {
        self.target
            .iter()
            .any(|c| c.cell == cell && c.target.is_option(option))
    }

    pub fn is_target_digit(&self, cell: Cell, digit: u8) -> bool {
        self.target
            .iter()
            .any(|c| c.cell == cell && c.target.is_digit(digit))
    }

    pub fn is_source(&self, cell: Cell) -> bool {
        self.source.iter().any(|c| c.cell == cell)
    }

    pub fn is_source_option(&self, cell: Cell, option: u8) -> bool {
        self.source
            .iter()
            .any(|c| c.cell == cell && c.target.is_option(option))
    }

    fn apply(&self, s: &mut Sudoku, c: &mut Options) {
        for t in &self.target {
            t.apply(s, c);
        }
    }
}

impl From<Solver> for StateMod {
    fn from(solver: Solver) -> Self {
        Self {
            solver,
            ..Default::default()
        }
    }
}

impl Default for StateMod {
    fn default() -> Self {
        Self {
            solver: Default::default(),
            source: Default::default(),
            target: Default::default(),
            marks: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ModTarget {
    Digit(u8),
    Option(u8),
    Cell,
}

impl Default for ModTarget {
    fn default() -> Self {
        Self::Cell
    }
}

impl ModTarget {
    fn is_option(&self, option: u8) -> bool {
        if let Self::Option(o) = self {
            return *o == option;
        }
        false
    }

    fn is_digit(&self, digit: u8) -> bool {
        if let Self::Digit(d) = self {
            return *d == digit
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base: Solver,
    pub solvers: Vec<Solver>,
    pub fallback: Option<Solver>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base: Solver::Base,
            solvers: vec![
                Solver::Single,
                Solver::Elim,
                Solver::Set,
                Solver::XWing,
                Solver::XYWing,
            ],
            fallback: Some(Solver::BackTrace),
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub sudoku: Sudoku,
    pub options: Options,
    pub info: Info,
    pub config: Rc<Config>,
}

impl State {
    pub fn update(&mut self, cell: Cell, value: u8) {
        self.sudoku.set_cell(cell, value);
    }

    pub fn remove(&mut self, cell: Cell, value: u8) -> bool {
        self.options.remove(cell, value)
    }

    pub fn merge_info(&mut self, other: &Self) {
        self.info.merge(&other.info);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            sudoku: Default::default(),
            options: Default::default(),
            info: Default::default(),
            config: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub mods: Vec<StateMod>,
    pub change: bool,
    pub tech: Solver,
    pub solved: bool,
    pub guesses: u32,
    pub guesses_t: u32,
    pub correct: bool,
    pub valid: bool,
}

impl Info {
    pub fn merge(&mut self, other: &Self) {
        self.guesses_t = other.guesses_t;
    }

    pub fn reset(&mut self) {
        self.mods = Vec::new();
        self.change = false;
        self.guesses = 0;
    }

    pub fn push_mod(&mut self, m: StateMod) {
        self.mods.push(m);
        self.change = true;
    }

    pub fn push_state(&mut self) {
        self.mods.push(StateMod::from(self.tech));
        self.change = true;
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            mods: Vec::new(),
            change: false,
            tech: Solver::Init,
            solved: false,
            guesses: 0,
            guesses_t: 0,
            correct: true,
            valid: true,
        }
    }
}

pub trait SolverExt {
    fn as_cloned_box(&self) -> Box<dyn EntrySolver>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn typename(&self) -> &str;
}

impl<T> SolverExt for T
where
    T: 'static + EntrySolver + Clone + Default,
{
    fn as_cloned_box(&self) -> Box<dyn EntrySolver> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn typename(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

impl Clone for Box<dyn EntrySolver> {
    fn clone(&self) -> Self {
        self.as_cloned_box()
    }
}

pub trait EntrySolver: SolverExt + std::fmt::Debug {
    fn advance(&mut self, state: &mut State) -> bool;
    fn verified(&self) -> bool {
        true
    }
    fn terminate(&self) -> bool {
        false
    }
}
