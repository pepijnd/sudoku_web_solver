use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::output::ser_array::a81;
use crate::output::Solve;
use crate::rules::Rules;
use crate::solving::{Entry, Info, Reporter, Target};
use crate::threading::SolveJobs;
use crate::util::Domain;
use crate::{AdvanceResult, Cell, Options, Solver};

#[derive(Debug, Clone)]
pub enum SolveResult {
    Invalid,
    Solution(Sudoku),
    Incomplete(Sudoku),
    Steps(Box<Solve>),
    List(Vec<Sudoku>),
    Jobs(Box<SolveJobs>),
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Sudoku {
    #[serde(with = "a81")]
    inner: [u8; 81],
}

impl Sudoku {
    pub fn solve(self, config: Option<Config>, reporter: Option<Reporter>) -> SolveResult {
        let reporter = reporter.unwrap_or_default();
        let config = config.unwrap_or_default();
        let buffer = Buffer::new(self, config);
        buffer.solve(reporter)
    }

    pub fn cell(&self, cell: Cell) -> &u8 {
        &self.inner[cell.index()]
    }

    pub fn cell_mut(&mut self, cell: Cell) -> &mut u8 {
        &mut self.inner[cell.index()]
    }

    pub fn set_cell(&mut self, cell: Cell, value: u8) {
        *self.cell_mut(cell) = value
    }

    pub fn row(&self, row: usize) -> SudokuIter {
        SudokuIter {
            sudoku: self,
            iter: Domain::Row(row),
            i: 0,
        }
    }

    pub fn col(&self, col: usize) -> SudokuIter {
        SudokuIter {
            sudoku: self,
            iter: Domain::Col(col),
            i: 0,
        }
    }

    pub fn sqr(&self, sqr: usize) -> SudokuIter {
        SudokuIter {
            sudoku: self,
            iter: Domain::Sqr(sqr),
            i: 0,
        }
    }

    pub fn as_string(self) -> String {
        let mut output = String::new();
        for cell in self.inner.iter() {
            if *cell > 0 {
                output.push_str(&format!("{}", cell));
            } else {
                output.push('.');
            }
        }
        output
    }

    pub fn inner(&self) -> &[u8] {
        &self.inner
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self { inner: [0; 81] }
    }
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for row in 0..9 {
            if row % 3 == 0 {
                writeln!(f)?;
            }
            for (col, cell) in self.row(row).enumerate() {
                if col % 3 == 0 {
                    write!(f, "|")?;
                }
                write!(f, "{}|", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> From<T> for Sudoku
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        assert!(input.as_ref().len() == 81);
        let mut sudoku = [0; 81];
        for (cell, output) in input.as_ref().chars().zip(sudoku.iter_mut()) {
            let value = cell.to_string().parse::<u8>();
            if let Ok(value) = value {
                if value > 0 && value <= 9 {
                    *output = value;
                }
            }
        }
        Sudoku { inner: sudoku }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SudokuIter<'a> {
    sudoku: &'a Sudoku,
    iter: Domain,
    i: usize,
}

impl<'a> Iterator for SudokuIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let out = if self.i == 9 {
            None
        } else {
            Some(*self.sudoku.cell(self.iter.cell(self.i)))
        };

        self.i += 1;
        out
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer: Vec<Entry>,
    pub rules: Rules,
}

impl Buffer {
    pub fn new(sudoku: Sudoku, config: Config) -> Self {
        let mut buffer = Vec::with_capacity(32);
        let rules = config.rules.clone();
        let state = Entry::new(sudoku, Options::default(), Solver::Init, config);
        buffer.push(state);
        Self { buffer, rules }
    }

    pub fn solve(mut self, mut reporter: Reporter) -> SolveResult {
        let mut solutions = Vec::new();
        loop {
            let entry = self.get().unwrap();
            let config = entry.config.clone();
            if
            /*config.canceled() ||*/
            solutions.len() > 1000 {
                match entry.config.target {
                    Target::Sudoku => {
                        return SolveResult::Incomplete(entry.sudoku);
                    }
                    Target::Steps => return SolveResult::Steps(Box::new(Solve::from(self))),
                    Target::List => return SolveResult::List(solutions),
                }
            }
            match entry.advance(&mut reporter) {
                AdvanceResult::Advance => {
                    let next = entry.make_next();
                    let entry = self.push(next).unwrap();
                    if entry.terminate() {
                        match config.target {
                            Target::Sudoku => {
                                return match entry.info {
                                    Info { valid: false, .. } => SolveResult::Invalid,
                                    Info { solved: true, .. } => {
                                        SolveResult::Solution(entry.sudoku)
                                    }
                                    Info { solved: false, .. } => {
                                        SolveResult::Incomplete(entry.sudoku)
                                    }
                                }
                            }
                            Target::Steps => {
                                return SolveResult::Steps(Box::new(Solve::from(self)))
                            }
                            Target::List => {
                                if entry.info.valid && entry.info.solved {
                                    solutions.push(entry.sudoku)
                                }
                            }
                        };
                    }
                }
                AdvanceResult::Invalid => {
                    let mut last_known = None;
                    loop {
                        let old = self.pop().unwrap();
                        if last_known.is_none() && old.info.correct {
                            last_known = Some(self.clone());
                        }
                        if let Some(entry) = self.get() {
                            if !entry.verified() {
                                break;
                            };
                        } else {
                            match config.target {
                                Target::Sudoku => {
                                    if let Some(last) = last_known.map(|mut b| b.pop()).flatten() {
                                        return SolveResult::Incomplete(last.sudoku);
                                    } else {
                                        return SolveResult::Invalid;
                                    }
                                }
                                Target::Steps => {
                                    if let Some(last) = last_known {
                                        return SolveResult::Steps(Box::new(Solve::from(last)));
                                    } else {
                                        return SolveResult::Steps(Box::new(Solve::invalid(
                                            old.sudoku,
                                            self.rules.clone(),
                                        )));
                                    }
                                }
                                Target::List => return SolveResult::List(solutions),
                            }
                        }
                    }
                }
                AdvanceResult::Split(jobs) => {
                    let split_depth = entry.info.splits;
                    self.pop();
                    return SolveResult::Jobs(Box::new(SolveJobs {
                        buffer: self,
                        jobs,
                        split_depth,
                    }));
                }
            }
        }
    }

    pub fn get(&mut self) -> Option<&mut Entry> {
        self.buffer.last_mut()
    }

    pub fn push(&mut self, state: Entry) -> Option<&mut Entry> {
        self.buffer.push(state);
        self.get()
    }

    pub fn pop(&mut self) -> Option<Entry> {
        self.buffer.pop()
    }

    pub fn into_inner(self) -> Vec<Entry> {
        self.buffer
    }
}

#[cfg(test)]
mod test {
    use crate::Sudoku;

    #[test]
    fn sudoku_solve_all() {
        let sudoku = Sudoku::from(
            "....27....1...4.....9..57...8....3..5..9..1......32...6.1....4...8....9.....4.6.5",
        );
        let _solutions = sudoku.solve(None, None);
        // assert_eq!(solutions.len(), 235);
    }
}
