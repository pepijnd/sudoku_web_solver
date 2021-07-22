use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use wasm_bindgen::JsValue;
use webelements::Result;

use crate::ui::app::AppController;
use crate::ui::editor::{Editor, EditorAction, EditorMode, EditorState};
use crate::util::InitCell;

#[derive(Debug, Clone)]
pub struct EditorController {
    element: Editor,
    pub app: InitCell<AppController>,
    pub state: Arc<Mutex<EditorState>>,
}

impl EditorController {
    pub fn update(&self) -> Result<()> {
        self.element.update(self)?;
        Ok(())
    }

    pub fn build(app: InitCell<AppController>, element: &Editor) -> Result<Self> {
        element.connect(InitCell::clone(&app.editor))?;
        app.element.rules.connect(InitCell::clone(&app.editor))?;

        element.steps.slider.slider.on_input(Box::new({
            let app = InitCell::clone(&app);
            let input = element.steps.slider.slider.clone();
            move |_event| {
                {
                    let mut info = app.info.info.lock().unwrap();
                    if let Ok(value) = input.get_value::<i32>() {
                        info.set_step(value.try_into().unwrap()).unwrap();
                    }
                }
                app.update().unwrap();
            }
        }))?;

        Ok(Self {
            app,
            element: element.clone(),
            state: Arc::new(Mutex::new(EditorState::default())),
        })
    }

    pub fn on_action(&self, action: EditorAction) -> Result<()> {
        let sudoku = &self.app.sudoku;
        if let EditorAction::Solve = action {
            sudoku.solve();
        } else {
            {
                let mut model = self.app.sudoku.state.borrow_mut();
                let mut info = self.app.info.info.lock().unwrap();
                let mut state = self.state.lock().unwrap();
                match action {
                    EditorAction::Erase => {
                        model.clear_state();
                        info.clear_solve()?;
                    }
                    EditorAction::Clear => {
                        model.clear_start();
                        model.clear_state();
                        model.clear_rules();
                        info.clear_solve()?;
                    }
                    EditorAction::SetValue(n) => match state.mode() {
                        EditorMode::Default => {
                            if let Some(cell) = model.selected() {
                                model.start_mut().set_cell(cell, n);
                            }
                        }
                        EditorMode::Cages => {
                            if let Some(cell) = state.sum_target() {
                                let id = model.rules.cages.cells[cell.index()];
                                if id != 0 {
                                    let total = model.rules.cages.cages[id - 1];
                                    let new = 10 * (total % 10) + n as u32;
                                    model.rules.cages.cages[id - 1] = new as u32;
                                }
                            }
                        }
                    },
                    EditorAction::First => {
                        if info.solve().is_some() {
                            info.set_step(0)?;
                        }
                    }
                    EditorAction::Prev => {
                        if info.solve().is_some() {
                            let step = info.step();
                            if step > 0 {
                                info.set_step(step - 1)?;
                            }
                        }
                    }
                    EditorAction::Next => {
                        if info.solve().is_some() {
                            let step = info.step();
                            if step < info.max() {
                                info.set_step(step + 1)?;
                            }
                        }
                    }
                    EditorAction::Last => {
                        if info.solve().is_some() {
                            let max = info.max();
                            info.set_step(max)?;
                        }
                    }
                    EditorAction::SetMode(mode) => {
                        state.set_sum_target(None);
                        state.set_mode(mode);
                    }
                    EditorAction::Clicked(cell) => {
                        if state.sum_target() == None {
                            if model.rules.cages.cells[cell.index()] == 0 {
                                model.rules.cages.cages.push(9);
                                let id = model.rules.cages.cages.len();
                                model.rules.cages.cells[cell.index()] = id;
                            } else {
                                let id = model.rules.cages.cells[cell.index()];
                                model.rules.cages.cells[cell.index()] = 0;
                                let mut clean = true;
                                for cell in model.rules.cages.cells.iter() {
                                    if *cell == id {
                                        clean = false;
                                    }
                                }
                                if clean {
                                    let last = model.rules.cages.cages.len();
                                    if last != id {
                                        for cell in model.rules.cages.cells.iter_mut() {
                                            if *cell == last {
                                                *cell = id;
                                            }
                                        }
                                        let total = model.rules.cages.cages.pop().unwrap();
                                        model.rules.cages.cages[id - 1] = total;
                                    }
                                }
                            }
                        }
                    }
                    EditorAction::Dragged(new) => {
                        if let Some(base) = state.drag() {
                            if state.sum_target() == None {
                                let id = model.rules.cages.cells[base.index()];
                                if id != 0 && model.rules.cages.cells[new.index()] == 0 {
                                    model.rules.cages.cells[new.index()] = id;
                                } else {
                                    state.set_drag(None);
                                }
                            }
                        }
                    }
                    EditorAction::CageSum(cell) => {
                        if let Some(cur) = state.sum_target() {
                            if cur == cell {
                                state.set_sum_target(None);
                            } else {
                                state.set_sum_target(Some(cell));
                            }
                        } else {
                            state.set_sum_target(Some(cell));
                        }
                    }
                    EditorAction::Solve | EditorAction::None => {}
                }
            }
            sudoku.app.update()?;
        }
        Ok(())
    }

    pub fn disabled(&self) -> bool {
        self.state.lock().unwrap().disabled()
    }

    pub fn mode(&self) -> EditorMode {
        self.state.lock().unwrap().mode()
    }
}
