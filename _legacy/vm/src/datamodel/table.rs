use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use super::{Identity, Tuple, Value, ValueType};

#[derive(Clone)]
pub struct Table {
    items: Rc<RefCell<Vec<(u64, Value)>>>,
}

impl Table {
    pub fn new(mut items: Vec<(u64, Value)>) -> Table {
        items.sort_unstable_by_key(|(k, _v)| *k);
        Table {
            items: Rc::from(RefCell::new(items)),
        }
    }

    pub fn to_vec(&self) -> Vec<Value> {
        self.items
            .borrow()
            .iter()
            .map(|(key, val)| {
                Tuple::new(vec![
                    RefCell::new((*key as i64).into()),
                    RefCell::new(val.clone()),
                ])
                .into()
            })
            .collect()
    }

    pub fn get(&self, key: u64) -> Option<Value> {
        let items = self.items.borrow();
        let index = items.binary_search_by_key(&key, |(k, _v)| *k).ok()?;
        let val = &unsafe { items.get_unchecked(index) }.1;
        Some(val.clone())
    }

    pub fn set(&self, key: u64, mut value: Value) -> Option<Value> {
        let mut items = self.items.borrow_mut();
        match items.binary_search_by_key(&key, |(k, _)| *k) {
            Ok(index) => {
                if value.get_type() == ValueType::None {
                    Some(items.remove(index).1)
                } else {
                    let item = &mut unsafe { items.get_unchecked_mut(index) }.1;
                    mem::swap(item, &mut value);
                    Some(value)
                }
            }
            Err(index) => {
                if value.get_type() != ValueType::None {
                    items.insert(index, (key, value));
                }
                None
            }
        }
    }
}

impl Identity for Table {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items).cast::<(u64, RefCell<Value>)>() as usize
    }
}
