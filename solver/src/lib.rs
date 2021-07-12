#![warn(missing_debug_implementations)]
#![feature(drain_filter)]
#![allow(unused_variables)]

pub mod config;
pub mod options;
pub mod output;
pub mod rules;
pub mod solvers;
pub mod solving;
pub mod sudoku;
pub mod threading;
pub mod util;

use config::Config;
use solving::{AdvanceResult, Reporter, State};
#[doc(inline)]
pub use {
    options::{CellOptions, Options},
    output::{Solve, SolveStep},
    solvers::Solver,
    sudoku::Sudoku,
    util::Cell,
};

pub trait EntrySolver: std::fmt::Debug + Send {
    fn advance(state: &mut State, config: &Config, reporter: &mut Reporter) -> AdvanceResult;
    fn verified(state: &State) -> bool {
        true
    }
    fn terminate() -> bool {
        false
    }
}
