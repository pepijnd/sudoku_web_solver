#![warn(missing_debug_implementations)]

pub mod config;
pub mod options;
pub mod output;
pub mod rules;
pub mod solvers;
pub mod solving;
pub mod sudoku;
pub mod threading;
pub mod util;

use solving::{AdvanceResult, SolverExt, State};
#[doc(inline)]
pub use {
    options::{CellOptions, Options},
    output::{Solve, SolveStep},
    solvers::Solver,
    sudoku::Sudoku,
    util::Cell,
};

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

pub trait EntrySolver: SolverExt + std::fmt::Debug + Send {
    fn advance(&mut self, state: &mut State) -> AdvanceResult;
    fn verified(&self) -> bool {
        true
    }
    fn terminate(&self) -> bool {
        false
    }
}
