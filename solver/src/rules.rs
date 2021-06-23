use crate::output::ser_array::a81;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rules {
    pub cages: Cages,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            cages: Cages::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cages {
    pub cages: Vec<u32>,
    #[serde(with = "a81")]
    pub cells: [usize; 81],
}

impl Default for Cages {
    fn default() -> Self {
        Self {
            cages: Vec::default(),
            cells: [0; 81],
        }
    }
}
