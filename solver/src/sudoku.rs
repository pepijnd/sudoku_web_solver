//!

use crate::{
    output::{ser_array::a81, Solve},
    rules::Rules,
    util::Domain,
    Cell, Config, Entry, Info, Options, Solver,
};

use serde::{Deserialize, Serialize};

/// Data structure that holds sudoku data.
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Sudoku {
    #[serde(with = "a81")]
    inner: [u8; 81],
}

impl Sudoku {
    pub fn solve(&self, config: Option<Config>) -> Solution {
        let config = config.unwrap_or_default();

        let mut buffer = Buffer::new(*self, config.clone());
        loop {
            let entry = buffer.get().unwrap();
            if config.canceled() {
                return Solution::Incomplete(entry.sudoku);
            }
            if entry.advance() {
                let next = entry.make_next();
                let entry = buffer.push(next).unwrap();
                if entry.terminate() {
                    return match entry.info {
                        Info { valid: false, .. } => Solution::Invalid,
                        Info { solved: true, .. } => Solution::Complete(entry.sudoku),
                        Info { solved: false, .. } => Solution::Incomplete(entry.sudoku),
                    };
                }
            } else {
                let mut last_known = None;
                loop {
                    let old = buffer.pop().unwrap();
                    if last_known.is_none() && old.info.correct {
                        last_known = Some(old.sudoku);
                    }
                    if let Some(entry) = buffer.get() {
                        entry.merge_info(&old);
                        if !entry.verified() {
                            break;
                        };
                    } else if let Some(last) = last_known {
                        return Solution::Incomplete(last);
                    } else {
                        return Solution::Invalid;
                    }
                }
            }
        }
    }

    pub fn solve_steps(&self, config: Option<Config>) -> Solve {
        let config = config.unwrap_or_default();

        let mut buffer = Buffer::new(*self, config.clone());
        loop {
            let entry = buffer.get().unwrap();
            if config.canceled() {
                return Solve::from(buffer);
            }
            if entry.advance() {
                let next = entry.make_next();
                let entry = buffer.push(next).unwrap();
                if entry.terminate() {
                    return Solve::from(buffer);
                }
            } else {
                let mut last_known = None;

                loop {
                    let old = buffer.pop().unwrap();
                    if last_known.is_none() && old.info.correct {
                        last_known = Some(buffer.clone());
                    }
                    if let Some(entry) = buffer.get() {
                        entry.merge_info(&old);
                        if !entry.verified() {
                            break;
                        };
                    } else if let Some(last) = last_known {
                        return Solve::from(last);
                    } else {
                        return Solve::invalid(*self, buffer.rules.clone());
                    }
                }
            }
        }
    }

    pub fn solve_all(&self, config: Option<Config>) -> Vec<Sudoku> {
        let mut solutions = Vec::new();
        let config = config.unwrap_or_default();

        let mut buffer = Buffer::new(*self, config.clone());
        loop {
            if solutions.len() >= 1000 || config.canceled() {
                return solutions;
            }

            let entry = buffer.get().unwrap();
            if entry.advance() {
                let next = entry.make_next();
                let entry = buffer.push(next).unwrap();
                if entry.terminate() && entry.info.valid && entry.info.solved {
                    solutions.push(entry.sudoku);
                }
            } else {
                loop {
                    let old = buffer.pop().unwrap();
                    if let Some(entry) = buffer.get() {
                        entry.merge_info(&old);
                        if !entry.verified() {
                            break;
                        };
                    } else {
                        return solutions;
                    }
                }
            }
        }
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

#[derive(Debug, Copy, Clone)]
pub enum Solution {
    Complete(Sudoku),
    Incomplete(Sudoku),
    Invalid,
}

#[cfg(test)]
mod test {
    use crate::Sudoku;

    #[test]
    fn sudoku_solve_all() {
        let sudoku = Sudoku::from(
            "....27....1...4.....9..57...8....3..5..9..1......32...6.1....4...8....9.....4.6.5",
        );
        let solutions = sudoku.solve_all(None);
        assert_eq!(solutions.len(), 235);
    }
}
