use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use solver::rules::Rules;
use solver::Solve;
use webelements::Result;

use super::app::AppController;
use crate::ui::sudoku::{Sudoku, SudokuModel, SudokuStateModel};
use crate::util::InitCell;

#[derive(Clone)]
pub struct SudokuController {
    element: Sudoku,
    pub app: InitCell<AppController>,
    pub solver: InitCell<Box<dyn Fn(solver::Sudoku, Rules)>>,
    pub state: Rc<RefCell<SudokuStateModel>>,
}

impl Debug for SudokuController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SudokuController")
            .field("app", &self.app)
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl SudokuController {
    pub fn update(&self) -> Result<()> {
        self.element.update(self)?;
        Ok(())
    }

    pub fn build(app: InitCell<AppController>, element: &Sudoku) -> Result<Self> {
        let sudoku = InitCell::clone(&app.sudoku);
        webelements::document()?
            .on_key(move |event| {
                {
                    let mut model = sudoku.state.borrow_mut();
                    let selected = model.selected();
                    if let Some(mut selected) = selected {
                        match &*event.key() {
                            "ArrowLeft" => {
                                if selected.col > 0 {
                                    selected.col -= 1
                                }
                            }
                            "ArrowUp" => {
                                if selected.row > 0 {
                                    selected.row -= 1
                                }
                            }
                            "ArrowRight" => {
                                if selected.col < 8 {
                                    selected.col += 1
                                }
                            }
                            "ArrowDown" => {
                                if selected.row < 8 {
                                    selected.row += 1
                                }
                            }
                            "Delete" => {
                                model.start_mut().set_cell(selected, 0);
                            }
                            str => {
                                if let Ok(value) = str.parse::<u8>() {
                                    if value <= 9 {
                                        model.start_mut().set_cell(selected, value);
                                    }
                                }
                            }
                        }
                        model.set_selected(selected);
                    }
                }
                sudoku.update().unwrap()
            })
            .unwrap();

        for cell in element.cells() {
            let clicked = cell.cell();
            let sudoku = InitCell::clone(&app.sudoku);
            cell.on_click(Box::new(move |_event| {
                {
                    let mut model = sudoku.state.borrow_mut();
                    model.set_selected(clicked);
                }
                sudoku.update().unwrap();
            }))?;
        }
        Ok(Self {
            app: InitCell::clone(&app),
            element: element.clone(),
            solver: InitCell::new(),
            state: Rc::new(RefCell::new(SudokuStateModel::default())),
        })
    }

    pub fn solve(&self) {
        let model = self.state.borrow();
        let start = model.start();
        (*self.solver)(*start.get(), model.rules.clone());
    }

    pub fn on_solve(&self, solve: Solve) -> Result<()> {
        {
            let mut model = self.state.borrow_mut();
            let mut info = self.app.info.info.lock().unwrap();

            let step = solve.iter().last().unwrap();
            model.set_state(SudokuModel::from(step.sudoku));
            info.set_solve(solve)?;
            let max = info.max();
            info.set_step(max)?;
        }
        self.app.update()?;
        Ok(())
    }

    pub fn set_solver(&self, solver: impl Fn(solver::Sudoku, Rules) + 'static) {
        InitCell::init(&self.solver, Box::new(solver))
    }
}
