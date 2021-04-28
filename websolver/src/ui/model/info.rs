use solver::{output::SolveStep, solvers::Solver, Options, Solve, StateMod, Sudoku};

use webelements::Result;

use crate::util::Measure;

#[derive(Debug, Clone, Copy)]
pub enum Stat {
    Tech,
    Steps,
    Guesses,
    GSteps,
    GTotal,
    None,
}

impl Default for Stat {
    fn default() -> Self {
        Self::None
    }
}

struct StateModStep<'a> {
    s_mod: &'a StateMod,
    sudoku: &'a Sudoku,
    cache: &'a Options,
}

#[derive(Debug)]
pub struct SudokuInfo {
    measure: Option<Measure>,
    solve: Option<Solve>,
    step: usize,
    s_step: Option<SolveStep>,
    max: usize,
}


impl SudokuInfo {
    pub fn measure(&self) -> Option<&Measure> {
        self.measure.as_ref()
    }

    pub fn set_measure(&mut self, m: Measure) {
        self.measure = Some(m);
    }

    pub fn solve(&self) -> Option<&Solve> {
        self.solve.as_ref()
    }

    pub fn set_solve(&mut self, s: Solve) -> Result<()> {
        let max = s.iter().count().saturating_sub(1);
        self.max = max;
        self.solve = Some(s);
        self.update_properties()?;
        Ok(())
    }

    pub fn clear_solve(&mut self) -> Result<()> {
        self.solve.take();
        self.s_step.take();
        self.max = 0;
        self.step = 0;
        self.update_properties()?;
        Ok(())
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn solve_step(&self) -> Option<&SolveStep> {
        self.s_step.as_ref()
    }

    pub fn set_step(&mut self, s: usize) -> Result<()> {
        if let Some(solve) = &self.solve {
            self.s_step = solve.iter().nth(s).cloned();
            self.step = s;
        }
        self.update_properties()?;
        Ok(())
    }

    pub fn update_properties(&self) -> Result<()> {
        let style = webelements::document()?.body()?.style();
        if self.solve().is_some() {
            style
                .set_property("--step-display", &format!("'{}'", self.step()))
                .unwrap();
            let ratio = 100.0 * (self.step() as f64 / self.max() as f64);
            style
                .set_property("--step-place", &format!("{}%", ratio))
                .unwrap();
        } else {
            style.set_property("--step-display", "'0'").unwrap();
            style.set_property("--step-place", "50%").unwrap();
        }
        Ok(())
    }

    pub fn property(&self, stat: Stat) -> Option<String> {
        let solve_step = self.solve_step();
        if let Some(step) = solve_step {
            match stat {
                Stat::Tech => Some(step.solver.to_string()),
                Stat::Steps => self
                    .solve()
                    .as_ref()
                    .map(|s| s.iter().count())
                    .map(|c| format!("{}", c)),
                Stat::Guesses => Some(format!("{}", step.guesses)),
                Stat::GTotal => Some(format!("{}", step.guesses_t)),
                Stat::GSteps => self
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
