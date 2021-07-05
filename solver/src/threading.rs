use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use crate::config::{Config, ConfigDescriptor};
use crate::rules::Rules;
use crate::solving::{Entry, Reporter, Target};
use crate::sudoku::{Buffer, SolveResult};
use crate::{Solver, Sudoku};

#[derive(Debug)]
pub struct RunnerJobs {
    buffer: Buffer,
    entries: Vec<Entry>,
    total: u32,
    size: u32,
}

#[derive(Debug, Clone)]
pub struct Runner {
    id: Option<usize>,
    queue: Arc<Mutex<Vec<RunnerJobs>>>,
    output: Arc<Mutex<Vec<Sudoku>>>,
    runners: Arc<Vec<AtomicBool>>,
    progress: Arc<Vec<Arc<Mutex<f64>>>>,
    global: Arc<Mutex<f64>>,
}

impl Runner {
    pub fn new(sudoku: Sudoku, rules: Rules) -> Self {
        let mut config_desc = ConfigDescriptor {
            rules,
            target: Target::List,
            max_splits: NonZeroU32::new(8 * 9 * 8),
            ..Default::default()
        };
        config_desc.add_rules_solvers();
        let config = Config::new(config_desc);
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
            total: 1,
            size: 1,
        });
        Self {
            id: None,
            queue: Arc::new(Mutex::new(queue)),
            output: Default::default(),
            runners: Arc::new((0..8).map(|_| AtomicBool::new(true)).collect()),
            progress: Arc::new((0..8).map(|_| Arc::new(Mutex::new(0.0))).collect()),
            global: Arc::new(Mutex::new(0.0))
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
        let mut reported = 0.0;
        loop {
            let mut done = true;
            for runner in self.runners.iter() {
                if runner.load(Ordering::Acquire) {
                    done = false;
                }
            }
            if done {
                break;
            }
            let mut progress = {*self.global.lock().unwrap()};
            // for job in self.queue.lock().unwrap().iter() {
            //     progress += job.retries.load(Ordering::Acquire) as f64 / (job.size * job.splits) as f64;
            // }
            for p in self.progress.iter() {
                progress += *p.lock().unwrap();
            }
            if progress > reported + 0.0005 {
                eprintln!("{:.2}%", progress.powi(8) * 100.0);
                reported = progress;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        for runner in runners {
            runner.join().unwrap()
        }
        self.output.lock().unwrap().clone()
    }

    fn thread_run(&self) {
        let mut status = true;
        let mut current = 1;
        let mut total = 1;
        loop {
            if let Some(job) = {
                let mut jobs = self.queue.lock().unwrap();
                jobs.retain(|f| !f.entries.is_empty());
                if let Some(job) = jobs.first_mut() {
                    if !status {
                        status = true;
                        eprintln!("thread: {} resumed", self.id.unwrap());
                        self.runners
                            .get(self.id.unwrap())
                            .unwrap()
                            .store(status, Ordering::Release);
                    }
                    let entry = job.entries.pop().unwrap();
                    let mut buffer = job.buffer.clone();
                    current = job.size;
                    total = job.total;
                    buffer.push(entry);
                    Some(buffer)
                } else {
                    None
                }
            } {
                let progress = Arc::clone(&self.progress[self.id.unwrap()]);
                match job.solve(Reporter::new(Box::new(move |p| {
                    *progress.lock().unwrap() = p;
                }))) {
                    SolveResult::List(ref solutions) => {
                        let mut output = self.output.lock().unwrap();
                        output.extend_from_slice(solutions);
                        *self.global.lock().unwrap() += 1.0 / (total) as f64;
                    }
                    SolveResult::Jobs(jobs) => {
                        let mut queue = self.queue.lock().unwrap();
                        let size = jobs.jobs.len() as u32;
                        queue.push(RunnerJobs {
                            buffer: jobs.buffer,
                            entries: jobs.jobs,
                            total: size * total,
                            size,
                        })
                    }
                    _ => {
                    }
                }
            } else {
                if status {
                    *self.progress[self.id.unwrap()].lock().unwrap() = 0.0;
                    eprintln!("thread: {} waiting", self.id.unwrap());
                }
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
    pub split_depth: u32,
}
