use std::cell::Ref;

use solver::{output::SolveStep, solvers::Solver, Options, Solve, StateMod, Sudoku};

use crate::{
    ui::{Model, UiModel},
    util::{body, Measure},
};

struct StateModStep<'a> {
    s_mod: &'a StateMod,
    sudoku: &'a Sudoku,
    cache: &'a Options,
}

#[derive(Debug, Clone)]
pub struct SudokuInfo {
    measure: Option<Measure>,
    solve: Option<Solve>,
    step: usize,
    s_step: Option<SolveStep>,
    max: usize,
}

impl UiModel for SudokuInfo {}

impl Model<SudokuInfo> {
    pub fn measure(&self) -> Ref<Option<Measure>> {
        let s = self.borrow();
        Ref::map(s, |s| &s.measure)
    }

    pub fn set_measure(&self, m: Measure) {
        crate::util::log(&format!("{}", &m));
        self.borrow_mut().measure = Some(m);
    }

    pub fn solve(&self) -> Ref<Option<Solve>> {
        let s = self.borrow();
        Ref::map(s, |s| &s.solve)
    }

    pub fn set_solve(&self, s: Solve) {
        let max = s.iter().count().saturating_sub(1);
        self.borrow_mut().max = max;
        self.borrow_mut().solve = Some(s);
        self.update_properties();
    }

    pub fn clear_solve(&self) {
        self.borrow_mut().solve = None;
        self.borrow_mut().max = 0;
        self.set_step(0);
        self.update_properties();
    }

    pub fn max(&self) -> Ref<usize> {
        let s = self.borrow();
        Ref::map(s, |s| &s.max)
    }

    pub fn step(&self) -> Ref<usize> {
        let s = self.borrow();
        Ref::map(s, |s| &s.step)
    }

    pub fn solve_step(&self) -> Ref<Option<SolveStep>> {
        let s = self.borrow();
        Ref::map(s, |s| &s.s_step)
    }

    pub fn set_step(&self, s: usize) {
        self.borrow_mut().step = s;
        let mut s_step = None;
        {
            let solve = self.solve();
            if let Some(solve) = solve.as_ref() {
                s_step = solve.iter().nth(s).cloned();
            }
        }
        self.borrow_mut().s_step = s_step;
        self.update_properties();
    }

    pub fn update_properties(&self) {
        let style = body().unwrap().style();
        if self.solve().is_some() {
            style
                .set_property("--step-display", &format!("'{}'", self.step()))
                .unwrap();
            let ratio = 100.0 * (*self.step() as f64 / *self.max() as f64);
            style
                .set_property("--step-place", &format!("{}%", ratio))
                .unwrap();
        } else {
            style.set_property("--step-display", "'0'").unwrap();
            style.set_property("--step-place", "50%").unwrap();
        }
    }

    pub fn property(&self, key: &str) -> Option<String> {
        let solve_step = self.solve_step().clone();
        if let Some(step) = solve_step {
            match key {
                "tech" => Some(step.solver.to_string()),
                "steps" => self
                    .solve()
                    .as_ref()
                    .map(|s| s.iter().count())
                    .map(|c| format!("{}", c)),
                "guess" => Some(format!("{}", step.guesses)),
                "guess_all" => Some(format!("{}", step.guesses_t)),
                "guess_steps" => self
                    .solve()
                    .as_ref()
                    .map(|s| s.iter().filter(|t| t.solver == Solver::BackTrace).count())
                    .map(|c| format!("{}", c)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Default for SudokuInfo {
    fn default() -> Self {
        Self {
            measure: None,
            solve: None,
            step: 0,
            s_step: None,
            max: 0,
        }
    }
}
