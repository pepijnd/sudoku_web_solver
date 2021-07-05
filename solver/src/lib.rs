#![warn(missing_debug_implementations)]
#![feature(drain_filter)]

pub mod config;
pub mod options;
pub mod output;
pub mod rules;
pub mod solvers;
pub mod solving;
pub mod sudoku;
pub mod threading;
pub mod util;

use solving::{AdvanceResult, Reporter, State};
#[doc(inline)]
pub use {
    options::{CellOptions, Options},
    output::{Solve, SolveStep},
    solvers::Solver,
    sudoku::Sudoku,
    util::Cell,
};

pub trait SolverExt {
    fn as_cloned_box(&self) -> Box<dyn EntrySolver>;
}

impl<T> SolverExt for T
where
    T: 'static + EntrySolver + Clone,
{
    fn as_cloned_box(&self) -> Box<dyn EntrySolver> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EntrySolver> {
    fn clone(&self) -> Self {
        self.as_cloned_box()
    }
}

pub trait EntrySolver: SolverExt + std::fmt::Debug + Send {
    fn advance(&mut self, state: &mut State, reporter: &mut Reporter) -> AdvanceResult;
    fn verified(&self) -> bool {
        true
    }
    fn terminate(&self) -> bool {
        false
    }
}
