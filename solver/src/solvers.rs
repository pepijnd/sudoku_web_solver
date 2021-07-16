use serde::{Deserialize, Serialize};

#[doc(inline)]
pub use self::{
    base::StateSolved,
    base::{Backtrace, BaseSolver, StateIncomplete, StateInit, StateInvalid},
    cage::CageSolver,
    elim::ElimSolver,
    sets::SetSolver,
    single::SingleSolver,
    xwing::XWingSolver,
    xywing::XYWingSolver,
};

use crate::{AdvanceResult, Config, EntrySolver, Reporter, State};

mod base;
mod cage;
mod elim;
mod sets;
mod single;
mod xwing;
mod xywing;

macro_rules! solver {
    {
        $vis:vis $name:ident {
            $var_init:ident => $solver_init:ident
            $(,$var:ident => $solver:ident)*$(,)?
        }
    } => {
        #[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
        $vis enum $name {
            $var_init,
            $($var),*
        }

        impl $name {
            pub fn advance(
                &self,
                state: &mut State,
                config: &Config,
                reporter: &mut Reporter
            ) -> AdvanceResult {
                match self {
                    Self::$var_init => {<$solver_init as EntrySolver>::advance(state, config, reporter)},
                    $(Self::$var => {
                        <$solver as EntrySolver>::advance(state, config, reporter)
                    }),*
                }
            }

            pub fn verified(&self, state: &State) -> bool {
                match self {
                    Self::$var_init => {<$solver_init as EntrySolver>::verified(state)},
                    $(Self::$var => {
                        <$solver as EntrySolver>::verified(state)
                    }),*
                }
            }

            pub fn terminate(&self) -> bool {
                match self {
                    Self::$var_init => {<$solver_init as EntrySolver>::terminate()},
                    $(Self::$var => {
                        <$solver as EntrySolver>::terminate()
                    }),*
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$var_init
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

solver! {
    pub Solver {
        Init => StateInit,
        BackTrace => Backtrace,
        Base => BaseSolver,
        Cage => CageSolver,
        Single => SingleSolver,
        Elim  => ElimSolver,
        Set => SetSolver,
        XWing => XWingSolver,
        XYWing => XYWingSolver,
        Invalid => StateInvalid,
        Incomplete => StateIncomplete,
        Solved => StateSolved,
    }
}
