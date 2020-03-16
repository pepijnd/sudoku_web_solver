use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use wasm_bindgen::JsValue;

use crate::ui::{Controller, Model, UiController, UiModel};

pub trait DynItemValue: std::fmt::Debug + Default {}

pub trait DynObj: std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T> DynObj for DynItem<T>
where
    T: 'static + DynItemValue,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct DynItem<T>
where
    T: DynItemValue,
{
    item: Rc<RefCell<T>>,
}

impl<T> std::ops::Deref for DynItem<T>
where
    T: DynItemValue,
{
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

unsafe impl<T> Sync for DynItem<T> where T: 'static + DynItemValue + Sync {}

impl<T> std::fmt::Debug for DynItem<T>
where
    T: DynItemValue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item.borrow())
    }
}

impl<T> Clone for DynItem<T>
where
    T: DynItemValue,
{
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
        }
    }
}

impl<T> Default for DynItem<T>
where
    T: DynItemValue,
{
    fn default() -> Self {
        Self {
            item: Default::default(),
        }
    }
}

impl<T> From<T> for DynItem<T>
where
    T: DynItemValue,
{
    fn from(item: T) -> Self {
        Self {
            item: Rc::new(RefCell::new(item)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynMap {
    items: Arc<Mutex<HashMap<&'static str, Box<dyn DynObj>>>>,
}

impl DynMap {
    pub fn new() -> DynMap {
        Self {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert<T>(&self, key: &'static str, item: DynItem<T>)
    where
        T: 'static + DynItemValue,
    {
        if let Ok(mut items) = self.items.lock() {
            items.insert(key, Box::new(item));
        }
    }

    pub fn get<T>(&self, key: &'static str) -> Result<DynItem<T>, JsValue>
    where
        T: 'static + DynItemValue,
    {
        if let Ok(items) = self.items.lock() {
            match items.get(key) {
                Some(model) => Self::downcast_item::<T>(model.as_ref())
                    .ok_or_else(|| JsValue::from_str("wrong type")),
                None => Err(JsValue::from_str("cannot find item")),
            }
        } else {
            Err(JsValue::from_str("cannot aquire lock"))
        }
    }

    pub fn init<T>(&self, key: &'static str) -> Model<T>
    where
        T: 'static + DynItemValue + UiModel,
    {
        let item: DynItem<T> = T::default().into();
        self.insert(key, item.clone());
        item
    }

    pub fn build<T>(&self, key: &'static str) -> Result<Controller<T>, JsValue>
    where
        T: 'static + DynItemValue + UiController,
    {
        let item: DynItem<T> = T::default().build()?;
        self.insert(key, item.clone());
        Ok(item)
    }

    fn downcast_item<E: 'static + DynItemValue>(item: &dyn DynObj) -> Option<DynItem<E>> {
        let item: Option<&DynItem<E>> = item.as_any().downcast_ref::<DynItem<E>>();
        match item {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }
}

unsafe impl Sync for DynMap {}
