use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::rules::Rules;
use crate::solving::Target;
use crate::Solver;

#[derive(Clone)]
pub struct Config {
    inner: Arc<ConfigDescriptor>,
}

impl std::ops::Deref for Config {
    type Target = ConfigDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: Arc::new(Default::default()),
        }
    }
}

impl Config {
    pub fn new(desc: ConfigDescriptor, _callback: Option<Callback>) -> Config {
        Config {
            inner: Arc::new(desc),
        }
    }
}

type Callback = Box<dyn Fn(&[(u32, u32)])>;
#[derive(Debug, Clone)]
pub struct ConfigDescriptor {
    pub base: Solver,
    pub solvers: Vec<Solver>,
    pub fallback: Option<Solver>,
    pub rules: Rules,
    pub target: Target,
    pub max_threading_depth: Option<NonZeroUsize>,
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

impl Default for ConfigDescriptor {
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
            max_threading_depth: None,
        }
    }
}

impl ConfigDescriptor {
    pub fn add_rules_solvers(&mut self) {
        if !self.rules.cages.cages.is_empty() {
            self.solvers.insert(0, Solver::Cage)
        }
    }
}
