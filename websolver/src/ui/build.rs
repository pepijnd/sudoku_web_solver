use std::{
    cell::Cell,
    num::NonZeroU64,
    sync::{Arc, Mutex},
};

use solver::{
    config::Config,
    rules::Rules,
    solving::{Entry, Target},
    sudoku::{Buffer, SolveResult},
    threading::{RunnerJobs, ThreadMessage},
    Solve, Solver, Sudoku,
};
use wasm_bindgen::prelude::*;
use webelements::{document, WebElementBuilder, Worker};

use super::controller::app::AppController;
use super::view::app::AppElement;
use crate::util::{InitCell, Measure};

#[derive(Debug)]
pub struct Runner {
    app: App,
    worker: JsValue,
    workers: Vec<(Worker, bool, u64, f64)>,
    queue: Vec<RunnerJobs>,
    working: bool,
    output: Vec<Sudoku>,
    config: Config,
    progress: f64,
    reported: f64,
}

impl Runner {
    fn new(app: App, worker: JsValue) -> Self {
        Self {
            app,
            worker,
            workers: Vec::new(),
            queue: Vec::new(),
            working: false,
            output: Vec::new(),
            config: Config::default(),
            progress: 0.0,
            reported: 0.0,
        }
    }
}

impl Runner {
    pub fn solve(&mut self, sudoku: Sudoku, rules: Rules) {
        if self.working {
            return;
        }
        self.working = true;


        self.config = Config {
            rules,
            target: Target::Steps,
            max_splits: NonZeroU64::new(8 * 9 * 8),
            ..Default::default()
        };
        self.config.add_rules_solvers();
        let mut buffer = Buffer::new(sudoku);
        let mut state = buffer
            .get()
            .expect("buffer always starts with at least one entry")
            .state
            .clone();
        state.info.entry.tech = Solver::NoOp;
        self.queue.push(RunnerJobs {
            buffer,
            entries: vec![Entry::from_state(state)],
            total: 1,
            size: 1,
        });

        let cpus = webelements::num_cpus().unwrap().max(1);
        webelements::log!("num_cpus: ", JsValue::from_serde(&cpus).unwrap());
        self.workers = (0..cpus)
            .map(|i| {
                let app = self.app.clone();
                Worker::new(&self.worker)
                    .map(|w| {
                        w.set_onmessage(move |value| app.on_worker_msg(i, value))
                            .map(|()| (w, false, 1, 0.0))
                    })
                    .flatten()
            })
            .collect::<Result<Vec<_>, _>>().unwrap();

        // let job = self.get_job().unwrap();
        // for (worker, ready, _, _) in self.workers.iter_mut() {
        //     if *ready {
        //         webelements::log!("runner: sending starting job");
        //         worker
        //             .post_message(JsValue::from_serde(&ThreadMessage::Job(Box::new(job))).unwrap())
        //             .unwrap();
        //         *ready = false;
        //         break;
        //     }
        // }
    }

    pub fn get_job(&mut self) -> Option<(solver::config::Config, solver::sudoku::Buffer)> {
        let job = if let Some(job) = self.queue.first_mut() {
            let entry = job.entries.pop().unwrap();
            let mut buffer = job.buffer.clone();
            buffer.push(entry);
            Some((self.config.clone(), buffer))
        } else {
            None
        };
        self.queue.retain(|f| !f.entries.is_empty());
        job
    }

    pub fn worker_msg(&mut self, worker: u32, msg: ThreadMessage) {
        let total = self.workers[worker as usize].2;
        match msg {
            ThreadMessage::Ready => {
                self.workers[worker as usize].1 = true;
            }
            ThreadMessage::Result(result) => {
                self.workers[worker as usize].1 = true;
                if !self.working {
                    return;
                }
                match result {
                    SolveResult::Invalid | SolveResult::Incomplete(_) => {}
                    SolveResult::Solution(_) => {}
                    SolveResult::Steps(solve) => {
                        if solve.solved() {
                            self.working = false;
                            self.app
                                .controller
                                .info
                                .info
                                .lock()
                                .unwrap()
                                .set_solve(*solve)
                                .unwrap();
                            self.app.controller.update().unwrap();
                        }
                    }
                    SolveResult::List(list) => {
                        self.output.extend_from_slice(&list[..]);
                        self.progress += 1.0 / total as f64;
                        self.workers[worker as usize].3 = 0.0;
                    }
                    SolveResult::Jobs(jobs) => {
                        let size = jobs.jobs.len() as u32;
                        self.queue.push(RunnerJobs {
                            buffer: jobs.buffer,
                            entries: jobs.jobs,
                            total: size as u64 * total,
                            size,
                        })
                    }
                }
            }
            ThreadMessage::Progress(p) => {
                self.workers[worker as usize].3 = p;
            }
            _ => {}
        }
        if !self.working {
            self.workers.drain(..).for_each(|(w, _, _, _)| {w.terminate()});
            return
        }
        let mut done = self.queue.is_empty();
        let mut progress = self.progress;
        for i in 0..self.workers.len() {
            progress += self.workers[i].3;
            if !self.workers[i].1 {
                done = false;
            } else if let Some(job) = self.get_job() {
                self.workers[i].1 = false;
                self.workers[i]
                    .0
                    .post_message(JsValue::from_serde(&ThreadMessage::Job(Box::new(job))).unwrap())
                    .unwrap();
                done = false;
            }
        }
        if progress > self.reported + 0.01 {
            self.app
                .controller
                .info
                .info
                .lock()
                .unwrap()
                .set_progress(progress)
                .unwrap();
            self.app.controller.info.update().unwrap();
            self.reported = progress;
        }
        if done {
            self.working = false;
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct App {
    controller: InitCell<AppController>,
    element: AppElement,
    runner: InitCell<Mutex<Runner>>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<App, JsValue> {
        let element = AppElement::build()?;
        let controller = element.controller()?;
        let app = Self {
            controller,
            element,
            runner: InitCell::new(),
        };
        let app_ref = app.clone();
        app.controller
            .sudoku
            .set_solver(move |sudoku, rules| app_ref.solve(sudoku, rules));
        Ok(app)
    }

    pub fn start(&self, worker: JsValue) -> Result<(), JsValue> {
        InitCell::init(&self.runner, Mutex::new(Runner::new(self.clone(), worker)));

        let sudoku = Sudoku::from(
            ".................................................................................",
        );

        let cages = solver::rules::Cages {
            cages: vec![20, 27, 26, 24, 28, 17, 18, 30, 16, 24],
            cells: [
                0, 0, 0, 0, 1, 2, 2, 2, 3, 0, 0, 0, 0, 1, 1, 1, 2, 3, 0, 0, 0, 0, 4, 4, 5, 5, 3, 0,
                0, 0, 0, 0, 4, 4, 5, 3, 6, 7, 8, 0, 0, 0, 4, 5, 3, 6, 7, 8, 8, 0, 0, 0, 0, 0, 6, 7,
                7, 8, 8, 0, 0, 0, 0, 6, 9, 10, 10, 10, 0, 0, 0, 0, 6, 9, 9, 9, 10, 0, 0, 0, 0,
            ],
        };

        {
            let mut state = self.controller.sudoku.state.borrow_mut();
            state.set_start(sudoku);
            state.rules.cages = cages;
        }

        self.controller
            .info
            .info
            .lock()
            .unwrap()
            .update_properties()
            .unwrap();
        self.controller.update()?;
        document()?.body()?.append(&self.element)?;
        Ok(())
    }

    pub fn on_worker_msg(&self, id: u32, msg: JsValue) {
        let msg = msg.into_serde().unwrap();
        self.runner.lock().unwrap().worker_msg(id, msg);
    }
}

impl App {
    pub fn solve(&self, sudoku: Sudoku, rules: Rules) {
        let mut runner = self.runner.lock().unwrap();
        runner.solve(sudoku, rules);
    }
}
