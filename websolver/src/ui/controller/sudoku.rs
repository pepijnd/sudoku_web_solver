use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use solver::rules::Rules;
use solver::Solve;
use webelements::Result;

use super::app::AppController;
use crate::ui::editor::{EditorAction, EditorMode};
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
        let editor = InitCell::clone(&app.editor);
        webelements::document()?
            .on_key(move |event| {
                {
                    if sudoku.app.editor.disabled() {
                        return;
                    }
                    let selected = {
                        let model = sudoku.state.borrow_mut();
                        model.selected()
                    };
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
                                sudoku.state.borrow_mut().start_mut().set_cell(selected, 0);
                            }
                            str => {
                                if let Ok(value) = str.parse::<u8>() {
                                    if value <= 9 {
                                        editor.on_action(EditorAction::SetValue(value)).unwrap();
                                    }
                                }
                            }
                        }
                        sudoku.state.borrow_mut().set_selected(selected);
                    }
                }
                sudoku.update().unwrap()
            })
            .unwrap();

        let editor = InitCell::clone(&app.editor);
        webelements::document()?.on_mouseup(move |_| {
            editor.state.lock().unwrap().set_drag(None);
        })?;

        for cell in element.cells() {
            let clicked = cell.cell();
            let sudoku = InitCell::clone(&app.sudoku);
            let editor = InitCell::clone(&app.editor);
            cell.on_click(move |_event| {
                match editor.mode() {
                    EditorMode::Default => {
                        if sudoku.app.editor.disabled() {
                            return;
                        }
                        let mut model = sudoku.state.borrow_mut();
                        if model.selected() == Some(clicked) {
                            model.deselect();
                        } else {
                            model.set_selected(clicked);
                        }
                    }
                    EditorMode::Cages => {
                        editor.on_action(EditorAction::Clicked(clicked)).unwrap();
                    }
                }
                sudoku.update().unwrap();
            })?;

            let editor = InitCell::clone(&app.editor);
            cell.on_mousedown(move |_| {
                if editor.mode() == EditorMode::Cages {
                    editor.state.lock().unwrap().set_drag(Some(clicked));
                }
            })?;

            let editor = InitCell::clone(&app.editor);
            cell.on_mouseenter(move |_| {
                if editor.mode() == EditorMode::Cages {
                    editor.on_action(EditorAction::Dragged(clicked)).unwrap();
                }
            })?;

            let editor = InitCell::clone(&app.editor);
            cell.bubble().on_click(move |e| {
                if editor.mode() == EditorMode::Cages {
                    editor.on_action(EditorAction::CageSum(clicked)).unwrap();
                    e.stop_propagation();
                }
            })?;
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
