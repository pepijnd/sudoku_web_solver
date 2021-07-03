use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::config::{Config, ConfigDescriptor};
use crate::rules::Rules;
use crate::solving::{Entry, Target};
use crate::sudoku::{Buffer, SolveResult};
use crate::{Solver, Sudoku};

#[derive(Debug)]
pub struct RunnerJobs {
    buffer: Buffer,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub struct Runner {
    id: Option<usize>,
    queue: Arc<Mutex<Vec<RunnerJobs>>>,
    output: Arc<Mutex<Vec<Sudoku>>>,
    runners: Arc<Vec<AtomicBool>>,
}

impl Runner {
    pub fn new(sudoku: Sudoku, rules: Rules) -> Self {
        let mut config_desc = ConfigDescriptor {
            rules,
            target: Target::List,
            max_threading_depth: NonZeroUsize::new(8),
            ..Default::default()
        };
        config_desc.add_rules_solvers();
        let config = Config::new(config_desc, None);
        let mut queue = Vec::new();
        let mut buffer = Buffer::new(sudoku, config);
        let mut state = buffer
            .get()
            .expect("buffer always starts with at least one entry")
            .state
            .clone();
        state.info.tech = Solver::NoOp;
        queue.push(RunnerJobs {
            buffer,
            entries: vec![Entry::from_state(state)],
        });
        Self {
            id: None,
            queue: Arc::new(Mutex::new(queue)),
            output: Default::default(),
            runners: Arc::new((0..128).map(|_| AtomicBool::new(true)).collect()),
        }
    }

    pub fn run(&self) -> Vec<Sudoku> {
        let runners = self
            .runners
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let runner = self.clone();
                std::thread::spawn(move || {
                    let runner = Self {
                        id: Some(i),
                        ..runner
                    };
                    runner.thread_run()
                })
            })
            .collect::<Vec<_>>();
        for runner in runners {
            runner.join().unwrap()
        }
        self.output.lock().unwrap().clone()
    }

    fn thread_run(&self) {
        let mut status = true;
        loop {
            if let Some(job) = {
                let mut jobs = self.queue.lock().unwrap();
                jobs.retain(|j| !j.entries.is_empty());
                if let Some(job) = jobs.first_mut() {
                    if !status {
                        status = true;
                        self.runners
                            .get(self.id.unwrap())
                            .unwrap()
                            .store(status, Ordering::Release);
                    }
                    let entry = job.entries.pop().unwrap();
                    let mut buffer = job.buffer.clone();
                    buffer.push(entry);
                    Some(buffer)
                } else {
                    None
                }
            } {
                match job.solve() {
                    SolveResult::List(ref solutions) => {
                        let mut output = self.output.lock().unwrap();
                        output.extend_from_slice(solutions)
                    }
                    SolveResult::Jobs(jobs) => {
                        let mut queue = self.queue.lock().unwrap();
                        queue.push(RunnerJobs {
                            buffer: jobs.buffer,
                            entries: jobs.jobs,
                        })
                    }
                    _ => {}
                }
            } else {
                status = false;
                self.runners
                    .get(self.id.unwrap())
                    .unwrap()
                    .store(status, Ordering::Release);
                let mut done = true;
                for runner in self.runners.iter() {
                    if runner.load(Ordering::Acquire) {
                        done = false;
                    }
                }
                if done {
                    break;
                }
                std::thread::yield_now();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolveJobs {
    pub buffer: Buffer,
    pub jobs: Vec<Entry>,
}
