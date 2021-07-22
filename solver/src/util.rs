use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn from_sqr(sqr: usize, i: usize) -> Self {
        Self {
            row: 3 * (sqr / 3) + i / 3,
            col: 3 * (sqr % 3) + i % 3,
        }
    }

    pub fn from_index(index: usize) -> Self {
        Self {
            row: index / 9,
            col: index % 9,
        }
    }

    pub fn sqr(&self) -> usize {
        3 * (self.row / 3) + self.col / 3
    }

    pub fn index(&self) -> usize {
        9 * self.row + self.col
    }

    pub fn sees(&self, other: Self) -> bool {
        self.row == other.row || self.col == other.col || self.sqr() == other.sqr()
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell: {{ row: {}, col: {} }}", self.row, self.col)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Domain {
    Sqr(usize),
    Row(usize),
    Col(usize),
}

impl Domain {
    pub fn domain(&self) -> SetDomain {
        match self {
            Domain::Sqr(_) => SetDomain::Sqr,
            Domain::Row(_) => SetDomain::Row,
            Domain::Col(_) => SetDomain::Col,
        }
    }

    fn set(&self) -> usize {
        match self {
            Domain::Sqr(n) | Domain::Row(n) | Domain::Col(n) => *n,
        }
    }

    pub fn cell(&self, i: usize) -> Cell {
        self.domain().cell(self.set(), i)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SetDomain {
    Sqr,
    Row,
    Col,
}

impl SetDomain {
    pub fn cell(&self, d: usize, i: usize) -> Cell {
        match self {
            SetDomain::Sqr => Cell::from_sqr(d, i),
            SetDomain::Row => Cell::new(d, i),
            SetDomain::Col => Cell::new(i, d),
        }
    }

    pub fn same(&self, a: Cell, b: Cell) -> bool {
        match self {
            SetDomain::Sqr => a.sqr() == b.sqr(),
            SetDomain::Row => a.row == b.row,
            SetDomain::Col => a.col == b.col,
        }
    }

    pub fn is(&self, c: Cell, i: usize) -> bool {
        match self {
            SetDomain::Sqr => c.sqr() == i,
            SetDomain::Row => c.row == i,
            SetDomain::Col => c.col == i,
        }
    }

    pub fn other(&self) -> Self {
        match self {
            SetDomain::Sqr => SetDomain::Sqr,
            SetDomain::Row => SetDomain::Col,
            SetDomain::Col => SetDomain::Row,
        }
    }

    pub fn matching(&self, c: Cell, i: usize) -> Cell {
        match self {
            SetDomain::Sqr => self.cell(c.sqr(), i),
            SetDomain::Row => self.cell(c.row, i),
            SetDomain::Col => self.cell(c.col, i),
        }
    }
}

pub enum CloneCell<T>
where
    T: Clone,
{
    Owned(T),
    Cloned(Arc<T>),
}

impl<T> CloneCell<T>
where
    T: Clone + Sized,
{
    pub fn disown(&mut self) {
        if matches!(self, Self::Cloned(_)) {
            return;
        }
        unsafe {
            let tmp = std::ptr::read(self);
            if let Self::Owned(tmp) = tmp {
                std::ptr::write(self, Self::Cloned(Arc::new(tmp)))
            } else {
                unreachable!()
            }
        }
    }
}

impl<T> Debug for CloneCell<T>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self).fmt(f)
    }
}

impl<T> Deref for CloneCell<T>
where
    T: Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            CloneCell::Owned(t) => t,
            CloneCell::Cloned(t) => t,
        }
    }
}

impl<T> DerefMut for CloneCell<T>
where
    T: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            CloneCell::Owned(t) => t,
            CloneCell::Cloned(_) => {
                *self = self.clone();
                if let Self::Owned(t) = self {
                    t
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl<T> Clone for CloneCell<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            CloneCell::Owned(t) => Self::Cloned(Arc::new(t.clone())),
            CloneCell::Cloned(t) => Self::Cloned(Arc::clone(t)),
        }
    }
}
