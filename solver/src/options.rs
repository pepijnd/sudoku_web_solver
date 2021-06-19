#![allow(clippy::suspicious_operation_groupings)]

use crate::{output::ser_array::a81, Cell, Sudoku};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, serde::Serialize, serde::Deserialize)]
pub struct CellOptions(u16);

impl CellOptions {
    pub fn all() -> Self {
        Self(0b1111111110)
    }

    #[inline(always)]
    pub fn add(&mut self, i: u8) {
        assert!(i <= 9, "{} !<= 9", i);
        self.0 |= 0x1 << i
    }

    #[inline(always)]
    pub fn remove(&mut self, i: u8) -> bool {
        assert!(i <= 9);
        let old = (self.0 & (0x1 << i)) >> i;
        self.0 &= !(0x1 << i);
        old != 0
    }

    #[inline(always)]
    pub fn has(&self, i: u8) -> bool {
        (self.0 & (0x1 << i)) >> i != 0
    }

    #[inline]
    pub fn take(&mut self) -> Option<u8> {
        for i in 1..=9 {
            let old = self.remove(i);
            if old {
                return Some(i);
            }
        }
        None
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        OptionsIter {
            options: self,
            i: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        let mut c = 0;
        for i in 1..=9 {
            if self.has(i) {
                c += 1;
            }
        }
        c
    }

    #[inline]
    pub fn sum(&self) -> u32 {
        let mut c = 0;
        for i in 1..=9 {
            if self.has(i) {
                c += i as u32;
            }
        }
        c
    }

    #[inline]
    pub fn found(&self) -> Option<u8> {
        let mut found = None;
        for i in 1..=9 {
            if self.has(i) {
                match found {
                    Some(_) => return None,
                    None => {
                        found = Some(i as u8);
                    }
                }
            }
        }
        found
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        for i in 1..=9 {
            if self.has(i) {
                return false;
            }
        }
        true
    }

    #[inline]
    pub fn is_set(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    pub fn combine(&mut self, other: &Self) {
        for i in 1..=9 {
            if other.has(i) {
                self.add(i);
            }
        }
    }

    pub fn as_pair(&self) -> Option<OptionPair> {
        let mut first = None;
        let mut second = None;
        for o in self.iter() {
            if first.is_none() {
                first = Some(o)
            } else if second.is_none() {
                second = Some(o)
            } else {
                return None;
            }
        }
        if let Some(first) = first {
            if let Some(second) = second {
                return Some(OptionPair(first, second));
            }
        }
        None
    }
}

impl std::fmt::Debug for CellOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, T> From<T> for CellOptions
where
    T: IntoIterator<Item = &'a u8>,
{
    fn from(input: T) -> Self {
        let mut options = Self::default();
        for i in input.into_iter() {
            options.add(*i);
        }
        options
    }
}

impl Default for CellOptions {
    fn default() -> Self {
        Self(0b0000000000)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OptionPair(u8, u8);

impl OptionPair {
    pub fn common(&self, other: Self) -> Option<u8> {
        // Note we compare self.0 to other.1
        if self.0 == other.0 || self.0 == other.1 {
            Some(self.0)
        } else if self.1 == other.0 || self.1 == other.1 {
            Some(self.1)
        } else {
            None
        }
    }
}

impl PartialEq for OptionPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 || self.0 == other.1) && (self.1 == other.0 || self.1 == other.1)
    }
}

pub struct OptionsIter<'a> {
    options: &'a CellOptions,
    i: u8,
}

impl<'a> Iterator for OptionsIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 9 {
            self.i += 1;
            if self.options.has(self.i) {
                return Some(self.i);
            } else {
                continue;
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Options {
    #[serde(with = "a81")]
    cells: [CellOptions; 81],
}

impl Options {
    pub fn remove(&mut self, cell: Cell, value: u8) -> bool {
        self.cells[cell.index()].remove(value)
    }

    pub fn options(&mut self, cell: Cell, sudoku: &Sudoku) -> CellOptions {
        let value = *sudoku.cell(cell);
        if value != 0 {
            let mut options = CellOptions::default();
            options.add(value);
            self.cells[cell.index()] = options;
            return options;
        }
        let options = &mut self.cells[cell.index()];
        for value in sudoku.row(cell.row) {
            options.remove(value);
        }
        for value in sudoku.col(cell.col) {
            options.remove(value);
        }
        for value in sudoku.sqr(cell.sqr()) {
            options.remove(value);
        }

        *options
    }

    pub fn cell(&self, cell: Cell) -> &CellOptions {
        &self.cells[9 * cell.row + cell.col]
    }

    pub fn cells(&self) -> &[CellOptions] {
        &self.cells
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cells: [CellOptions::all(); 81],
        }
    }
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..9 {
            if row % 3 == 0 {
                writeln!(f)?;
            }
            for col in 0..9 {
                if col % 3 == 0 {
                    write!(f, "|")?;
                }
                write!(f, "{}|", self.cell(Cell { row, col }).len())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Cell, CellOptions, Sudoku};

    use super::Options;

    #[test]
    fn options_all() {
        let options = CellOptions::all();
        assert_eq!(
            options.iter().collect::<Vec<u8>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn options_none() {
        let options = CellOptions::default();
        assert_eq!(options.iter().collect::<Vec<u8>>(), vec![]);
    }

    #[test]
    fn options_add() {
        let mut options = CellOptions::default();
        options.add(2);
        options.add(5);
        options.add(7);
        options.add(9);
        assert_eq!(options.iter().collect::<Vec<u8>>(), vec![2, 5, 7, 9]);
    }

    #[test]
    fn options_remove() {
        let mut options = CellOptions::all();
        options.remove(2);
        options.remove(5);
        options.remove(7);
        options.remove(9);
        assert_eq!(options.iter().collect::<Vec<u8>>(), vec![1, 3, 4, 6, 8]);
    }

    #[test]
    fn options_set() {
        let mut superset = CellOptions::default();
        superset.add(1);
        superset.add(3);
        superset.add(6);
        superset.add(8);
        let mut subset = CellOptions::default();
        subset.add(1);
        subset.add(3);
        subset.add(6);
        assert!(superset.is_set(&subset));
        subset.add(7);
        assert!(!superset.is_set(&subset));
    }

    static SAMPLE: &str =
        "___________98____7_8__6__5__5__4__3___79____2___________27____9_4__5__6_3____62__";

    #[test]
    fn cache_string() {
        let cache = Options::default();
        cache.to_string();
    }

    #[test]
    fn cache_options() {
        let sudoku = Sudoku::from(SAMPLE);
        let mut cache = Options::default();
        let options = CellOptions::from(&[1, 2, 4, 5, 6, 7]);
        assert_eq!(cache.options(Cell::new(0, 0), &sudoku), options);
    }
}
