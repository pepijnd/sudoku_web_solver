use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use webelements::Result;

use crate::ui::app::AppController;
use crate::ui::editor::{Editor, EditorAction, EditorState};
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
            state: Arc::new(Mutex::new(EditorState::default()))
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
                match action {
                    EditorAction::Erase => {
                        model.clear_state();
                        info.clear_solve()?;
                    }
                    EditorAction::Clear => {
                        model.clear_start();
                        model.clear_state();
                        info.clear_solve()?;
                    }
                    EditorAction::SetValue(n) => {
                        if let Some(cell) = model.selected() {
                            model.start_mut().set_cell(cell, n);
                        }
                    }
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
                    _ => {}
                }
            }
            sudoku.app.update()?;
        }
        Ok(())
    }

    pub fn disabled(&self) -> bool {
        self.state.lock().unwrap().disabled()
    }
}
