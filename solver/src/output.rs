use serde::{Deserialize, Serialize};

use crate::solving::{EntryInfo, Info, StateMod};
use crate::sudoku::Buffer;
use crate::{Options, Solver, Sudoku};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solve {
    steps: Vec<SolveStep>,
}

impl Solve {
    pub fn iter(&self) -> impl Iterator<Item = &SolveStep> {
        self.steps.iter()
    }

    pub fn end(&self) -> &SolveStep {
        self.steps
            .last()
            .expect("Solve always has at least one step")
    }

    pub fn solved(&self) -> bool {
        if let Some(step) = self.steps.last() {
            return step.solved;
        }
        false
    }

    pub fn invalid(sudoku: Sudoku) -> Self {
        let cache = Options::default();
        Self {
            steps: vec![SolveStep {
                sudoku,
                cache,
                solver: Solver::Incomplete,
                change: StateMod::default(),
                solved: false,
                correct: true,
                valid: false,
            }],
        }
    }

    pub fn from_buffer(buffer: Buffer) -> Self {
        Self {
            steps: buffer
                .into_inner()
                .into_iter()
                .filter(|s| { s.info.entry.change && !s.info.mods.is_empty() })
                .scan(None, |s, e| {
                    let solver = e.solver;
                    let sudoku = e.state.sudoku;
                    let cache = e.state.options;
                    let Info {
                        mods,
                        entry:
                            EntryInfo {
                                solved,
                                correct,
                                valid,
                                ..
                            },
                        ..
                    } = e.state.info;
                    let (sudoku, cache) = s.replace((sudoku, cache)).unwrap_or((sudoku, cache));
                    Some(mods.into_iter().scan(None, move |s, m| {
                        s.get_or_insert((sudoku, cache));
                        let (sudoku, cache) = s.unwrap();
                        s.iter_mut().for_each(|(s, c)| m.apply(s, c));
                        Some(SolveStep {
                            sudoku,
                            cache,
                            solver,
                            change: m,
                            solved,
                            correct,
                            valid,
                        })
                    }))
                })
                .flatten()
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolveStep {
    pub sudoku: Sudoku,
    pub cache: Options,
    pub solver: Solver,
    pub change: StateMod,
    pub solved: bool,
    pub correct: bool,
    pub valid: bool,
}

#[doc(hidden)]
pub fn serialize_array<S, T>(array: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::ser::Serialize,
{
    array.serialize(serializer)
}

#[doc(hidden)]
#[macro_export]
macro_rules! serde_array {
    ($m:ident, $n:expr) => {
        pub mod $m {
            use std::{mem, ptr};

            use serde::{de, Deserialize, Deserializer};

            pub use crate::output::serialize_array as serialize;

            pub fn deserialize<'de, D, T>(deserializer: D) -> Result<[T; $n], D::Error>
            where
                D: Deserializer<'de>,
                T: Deserialize<'de> + 'de,
            {
                let slice: Vec<T> = Deserialize::deserialize(deserializer)?;
                if slice.len() != $n {
                    return Err(de::Error::custom("input slice has wrong length"));
                }
                unsafe {
                    let mut result: [T; $n] = mem::MaybeUninit::uninit().assume_init();
                    for (src, dst) in slice.into_iter().zip(&mut result[..]) {
                        ptr::write(dst, src);
                    }
                    Ok(result)
                }
            }
        }
    };
}

#[doc(hidden)]
pub mod ser_array {
    serde_array!(a81, 81);
}
