use std::{cell::RefCell, rc::Rc};

use webelements::Result;

use crate::{
    ui::{view::info::Info, SudokuInfo},
    util::InitCell,
};

use super::app::AppController;

#[derive(Debug, Clone)]
pub struct InfoController {
    element: Info,
    app: InitCell<AppController>,
    pub info: Rc<RefCell<SudokuInfo>>,
}

impl InfoController {
    pub fn update(&self) -> Result<()> {
        self.element.update(self)?;
        Ok(())
    }
    pub fn build(app: InitCell<AppController>, info: &Info) -> Result<Self> {
        Ok(Self {
            app,
            element: info.clone(),
            info: Rc::new(RefCell::new(SudokuInfo::default())),
        })
    }
}
