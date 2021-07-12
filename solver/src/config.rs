use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use crate::rules::Rules;
use crate::solving::Target;
use crate::Solver;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub base: Solver,
    pub solvers: Vec<Solver>,
    pub fallback: Option<Solver>,
    pub rules: Rules,
    pub target: Target,
    pub max_splits: Option<NonZeroU64>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("base", &self.base)
            .field("solvers", &self.solvers)
            .field("fallback", &self.fallback)
            .field("rules", &self.rules)
            .finish_non_exhaustive()
    }
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
            rules: Rules::default(),
            target: Target::Steps,
            max_splits: None,
        }
    }
}

impl Config {
    pub fn add_rules_solvers(&mut self) {
        if !self.rules.cages.cages.is_empty() {
            self.solvers.insert(0, Solver::Cage)
        }
    }
}
