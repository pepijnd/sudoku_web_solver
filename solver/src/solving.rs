use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::util::Domain;
use crate::{Cell, CellOptions, Options, Solver, Sudoku};

#[derive(Debug, Copy, Clone)]
pub enum Solution {
    Complete(Sudoku),
    Incomplete(Sudoku),
    Invalid,
}

#[derive(Debug)]
pub enum AdvanceResult {
    Advance,
    Invalid,
    Split(Vec<Entry>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub state: State,
    pub solver: Solver,
}

impl Entry {
    pub fn new(sudoku: Sudoku, options: Options, tech: Solver) -> Self {
        Self {
            state: State {
                sudoku,
                options,
                ..Default::default()
            },
            solver: tech,
        }
    }

    pub fn from_state(state: State) -> Self {
        let tech = state.info.entry.tech;
        Self {
            state,
            solver: tech,
        }
    }

    pub fn make_next(&self, config: &Config) -> Entry {
        let mut state = self.state.clone();
        state.info.reset();

        if self.info.entry.solved {
            for tech in &config.solvers {
                let mut test_state = state.clone();
                test_state.info.entry.tech = *tech;
                let mut entry = Entry::from_state(test_state);
                if !matches!(
                    entry.advance(config, &mut Reporter::default()),
                    AdvanceResult::Advance
                ) {
                    state.info.entry.tech = Solver::Invalid;
                    state.info.push_state();
                    return Entry::from_state(state);
                }
            }
            state.info.entry.tech = Solver::Solved;
            state.info.push_state();
            return Entry::from_state(state);
        }

        if self.info.entry.change {
            state.info.entry.tech = config.base;
            return Entry::from_state(state);
        }

        let mut next = false;
        for &tech in config.solvers.iter() {
            if self.solver == config.base || next {
                state.info.entry.tech = tech;
                return Entry::from_state(state);
            } else if tech == self.solver {
                next = true;
            }
        }

        if let Some(tech) = config.fallback {
            state.info.entry.tech = tech;
            Entry::from_state(state)
        } else {
            state.info.entry.tech = Solver::Incomplete;
            state.info.push_state();
            Entry::from_state(state)
        }
    }

    pub fn advance(&mut self, config: &Config, reporter: &mut Reporter) -> AdvanceResult {
        self.state.info.entry.tech = self.solver;
        self.solver.advance(&mut self.state, config, reporter)
    }

    pub fn verified(&self, state: &State) -> bool {
        self.solver.verified(state)
    }

    pub fn terminate(&self) -> bool {
        self.solver.terminate()
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
    pub fn digit(cell: Cell, digit: u8) -> CellMod {
        CellMod {
            cell,
            target: ModTarget::Digit(digit),
        }
    }

    pub fn option(cell: Cell, option: u8) -> CellMod {
        CellMod {
            cell,
            target: ModTarget::Option(option),
        }
    }

    pub fn apply(&self, s: &mut Sudoku, c: &mut Options) -> bool {
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
    Cage(u32),
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

    pub fn targets(&self) -> impl Iterator<Item = &CellMod> {
        self.target.iter()
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

    pub fn from_change(solver: Solver, cell: Cell, value: u8) -> Self {
        Self {
            solver,
            source: smallvec::smallvec![],
            target: smallvec::smallvec![CellMod {
                cell,
                target: ModTarget::Digit(value)
            }],
            marks: smallvec::smallvec![],
        }
    }

    pub fn apply(&self, s: &mut Sudoku, c: &mut Options) {
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
            return *d == digit;
        }
        false
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Target {
    Sudoku,
    Steps,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktraceInfo {
    pub cell: Option<Cell>,
    pub options: CellOptions,
    pub retries: u32,
    pub job: bool,
}

impl Default for BacktraceInfo {
    fn default() -> Self {
        Self {
            cell: None,
            options: CellOptions::default(),
            retries: 0,
            job: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caches {}

impl Default for Caches {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub sudoku: Sudoku,
    pub options: Options,
    pub info: Info,
    pub caches: Caches,
}

impl State {
    pub fn update(&mut self, cell: Cell, value: u8) {
        self.sudoku.set_cell(cell, value);
    }

    pub fn remove(&mut self, cell: Cell, value: u8) -> bool {
        self.options.remove(cell, value)
    }

    pub fn options(&mut self, cell: Cell) -> CellOptions {
        self.options.options(cell, &self.sudoku)
    }
}

impl Default for State {
    fn default() -> State {
        Self {
            sudoku: Default::default(),
            options: Default::default(),
            info: Default::default(),
            caches: Caches::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntryInfo {
    pub change: bool,
    pub tech: Solver,
    pub solved: bool,
    pub correct: bool,
    pub valid: bool,
    pub depth: u32,
    pub splits: u32,
}

impl Default for EntryInfo {
    fn default() -> Self {
        Self {
            change: false,
            tech: Solver::Init,
            solved: false,
            correct: true,
            valid: true,
            depth: 0,
            splits: 1,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub mods: Vec<StateMod>,
    pub backtrace: Option<BacktraceInfo>,
    pub entry: EntryInfo,
}

impl Info {
    pub fn reset(&mut self) {
        self.mods = Vec::new();
        self.backtrace = None;
        self.entry.change = false;
    }

    pub fn push_mod(&mut self, m: StateMod) {
        self.mods.push(m);
        self.entry.change = true;
    }

    pub fn push_state(&mut self) {
        self.mods.push(StateMod::from(self.entry.tech));
        self.entry.change = true;
    }

    pub fn backtrace(&mut self) -> &mut BacktraceInfo {
        if self.backtrace.is_none() {
            self.backtrace.replace(BacktraceInfo::default());
        }
        if let Some(info) = self.backtrace.as_mut() {
            info
        } else {
            unreachable!()
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            mods: Vec::new(),
            backtrace: None,
            entry: EntryInfo::default(),
        }
    }
}

#[derive(Debug)]
pub struct Progress {
    retries: u32,
    splits: u32,
}

pub struct Reporter {
    progress: Vec<Progress>,
    on_progress: Option<Box<dyn FnMut(f64) + 'static>>,
}

impl Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reporter").finish_non_exhaustive()
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self {
            progress: Vec::new(),
            on_progress: None,
        }
    }
}

impl Reporter {
    pub fn new(on_progress: Box<dyn FnMut(f64)>) -> Self {
        Self {
            on_progress: Some(on_progress),
            ..Default::default()
        }
    }

    pub fn progress(&mut self, retries: u32, splits: u32) {
        if let Some(callback) = self.on_progress.as_mut() {
            let mut updated = false;
            self.progress.drain_filter(|p| match p.splits.cmp(&splits) {
                std::cmp::Ordering::Less => false,
                std::cmp::Ordering::Equal => {
                    p.retries = retries;
                    updated = true;
                    false
                }
                std::cmp::Ordering::Greater => true,
            });
            if !updated {
                self.progress.push(Progress { retries, splits });
            }
            let mut progress = 0.0;
            for p in &self.progress {
                // dbg!(p.retries, p.splits);
                progress += p.retries as f64 / p.splits as f64;
            }
            callback(progress);
        }
    }
}
