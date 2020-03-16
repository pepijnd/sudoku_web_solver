use solver::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement, MouseEvent};

use crate::util::NodeExt;
use crate::{
    element,
    ui::{model::sudoku::SudokuStateModel, SudokuInfo},
};
use crate::{ui::models, util::ElementExt};

#[derive(Debug, Clone)]
pub struct SudokuElement {
    element: HtmlDivElement,
    cells: Box<[CellElement]>,
}

impl AsRef<Element> for SudokuElement {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}

impl SudokuElement {
    pub fn new() -> Result<SudokuElement, JsValue> {
        let element = element!(div "sdk-sudoku")?;
        let cells = (0..81)
            .map(|i| {
                let cell = Cell::new(i / 9, i % 9);
                let cell = CellElement::new(cell).unwrap();
                element.append_child(cell.as_ref()).unwrap();
                cell
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(Self { element, cells })
    }

    pub fn deep_clone(&self) -> Result<Self, JsValue> {
        Ok(Self {
            element: self.element.deep_clone()?,
            cells: self
                .cells
                .iter()
                .map(|e| e.deep_clone())
                .collect::<Result<Box<[_]>, _>>()?,
        })
    }

    pub fn cells(&self) -> std::slice::Iter<CellElement> {
        self.cells.iter()
    }

    pub fn update(&self) {
        for cell in self.cells.iter() {
            cell.update();
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellElement {
    element: HtmlDivElement,
    number: HtmlDivElement,
    options: OptionsElement,
    cell: Cell,
}

impl AsRef<Element> for CellElement {
    fn as_ref(&self) -> &Element {
        &self.element
    }
}

impl CellElement {
    pub fn new(cell: Cell) -> Result<CellElement, JsValue> {
        let element = element!(div "sdk-cell")?;
        let number = element!(div "sdk-number")?;
        let options = OptionsElement::new(cell)?;
        element.append_child(&number)?;
        element.append_child(options.as_ref())?;
        Ok(Self {
            element,
            number,
            options,
            cell,
        })
    }

    pub fn deep_clone(&self) -> Result<Self, JsValue> {
        Ok(Self {
            element: self.element.deep_clone()?,
            number: self.number.deep_clone()?,
            options: self.options.deep_clone()?,
            cell: self.cell,
        })
    }

    pub fn cell(&self) -> Cell {
        self.cell
    }

    pub fn update(&self) {
        let model = models().get::<SudokuStateModel>("sudoku").unwrap();
        let info = models().get::<SudokuInfo>("info").unwrap();
        let step = info
            .solve_step()
            .as_ref()
            .map(|s| *s.sudoku.cell(self.cell));

        let value = model.start().cell(self.cell);
        debug_assert!(value <= 9, format!("invalid cell value {}", value));
        self.number.remove_class("starting state empty");
        self.remove_class("selected");

        self.remove_class("target");
        self.remove_class("source");
        if let Some(step) = info.solve_step().as_ref() {
            if step.change.is_target(self.cell) {
                self.add_class("target");
            } else if step.change.is_source(self.cell) {
                self.add_class("source");
            }
        }

        if info.solve().is_some() {
            self.options.remove_class("hidden");
        } else {
            self.options.add_class("hidden");
        }
        if value > 0 {
            self.number.set_text(&format!("{}", value));
            self.number.add_class("starting");
            self.options.add_class("hidden");
        } else if let Some(value) = step {
            self.number.add_class("state");
            if value > 0 {
                self.number.set_text(&format!("{}", value));
                self.options.add_class("hidden");
            } else {
                self.number.set_text("");
            }
        } else {
            self.number.add_class("empty");
            self.number.set_text("");
        }
        if let Some(selected) = model.selected() {
            if selected == self.cell {
                self.add_class("selected");
            }
        }
        self.options.update();
    }

    pub fn on_click(&self, closure: Box<dyn FnMut(MouseEvent)>) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct OptionsElement {
    element: HtmlDivElement,
    options: Box<[HtmlDivElement]>,
    cell: Cell,
}

impl OptionsElement {
    fn new(cell: Cell) -> Result<Self, JsValue> {
        let element = element!(div "cell-options")?;
        let options = (0..9)
            .map(|i| {
                let cell = element!(div "cell-option").unwrap();
                element.append_child(&cell).unwrap();
                cell.set_text(&format!("{}", i + 1));
                cell
            })
            .collect::<Box<[_]>>();
        let element = Self {
            element,
            options,
            cell,
        };
        element.update();
        Ok(element)
    }

    pub fn deep_clone(&self) -> Result<Self, JsValue> {
        Ok(Self {
            element: self.element.deep_clone()?,
            options: self
                .options
                .iter()
                .map(|e| e.deep_clone())
                .collect::<Result<Box<[_]>, _>>()?,
            cell: self.cell,
        })
    }

    fn update(&self) {
        let info = models().get::<SudokuInfo>("info").unwrap();
        for (option, e) in self.options.iter().enumerate() {
            if let Some(step) = info.solve_step().as_ref() {
                let index = option as u8 + 1;
                let mut cache = step.cache;
                e.remove_class("target");
                e.remove_class("source");
                if cache.options(self.cell, &step.sudoku).has(index) {
                    e.remove_class("hidden");
                } else {
                    e.add_class("hidden");
                }
                if let Some(step) = info.solve_step().as_ref() {
                    if step.change.is_target_option(self.cell, index) {
                        e.remove_class("hidden");
                        e.add_class("target")
                    } else if step.change.is_source_option(self.cell, index) {
                        e.remove_class("hidden");
                        e.add_class("source")
                    }
                }
            }
        }
    }
}

impl AsRef<Element> for OptionsElement {
    fn as_ref(&self) -> &Element {
        self.element.as_ref()
    }
}
