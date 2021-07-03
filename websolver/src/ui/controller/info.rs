use std::cell::RefCell;
use std::rc::Rc;

use webelements::Result;

use super::app::AppController;
use crate::ui::view::info::Info;
use crate::ui::SudokuInfo;
use crate::util::InitCell;

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
