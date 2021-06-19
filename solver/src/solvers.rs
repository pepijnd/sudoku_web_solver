use base::StateIncomplete;

use crate::EntrySolver;

use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use self::{
    base::StateSolved,
    base::{Backtrace, BaseSolver, StateInit},
    cage::CageSolver,
    elim::ElimSolver,
    sets::SetSolver,
    single::SingleSolver,
    xwing::XWingSolver,
    xywing::XYWingSolver,
};

mod base;
mod cage;
mod elim;
mod sets;
mod single;
mod xwing;
mod xywing;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Solver {
    Init,
    BackTrace,
    Base,
    Cage,
    Single,
    Elim,
    Set,
    XWing,
    XYWing,
    Incomplete,
    Solved,
}

impl Default for Solver {
    fn default() -> Self {
        Self::Init
    }
}

impl Solver {
    pub fn make(&self) -> Box<dyn EntrySolver> {
        match self {
            Solver::Init => Box::new(StateInit::default()),
            Solver::BackTrace => Box::new(Backtrace::default()),
            Solver::Base => Box::new(BaseSolver::default()),
            Solver::Cage => Box::new(CageSolver::default()),
            Solver::Single => Box::new(SingleSolver::default()),
            Solver::Elim => Box::new(ElimSolver::default()),
            Solver::Set => Box::new(SetSolver::default()),
            Solver::XWing => Box::new(XWingSolver::default()),
            Solver::XYWing => Box::new(XYWingSolver::default()),
            Solver::Incomplete => Box::new(StateIncomplete::default()),
            Solver::Solved => Box::new(StateSolved::default()),
        }
    }
}

impl std::fmt::Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
