use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;

use super::{Identity, Value};

#[derive(Clone)]
pub struct List {
    items: Rc<RefCell<Vec<Value>>>,
}

impl List {
    pub fn new(v: Vec<Value>) -> List {
        List {
            items: Rc::new(RefCell::new(v)),
        }
    }

    pub fn empty() -> List {
        List::new(Vec::new())
    }

    pub fn as_slice(&self) -> Ref<[Value]> {
        Ref::map(self.items.borrow(), |items| &items[..])
    }

    pub fn len(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn resize(&self, len: usize) {
        let mut items = self.items.borrow_mut();
        items.resize_with(len, || Value::None);
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        let items = self.items.borrow();
        let item = items.get(index)?;
        Some(item.clone())
    }

    pub fn set(&self, index: usize, mut value: Value) -> Option<Value> {
        let mut items = self.items.borrow_mut();
        let item = items.get_mut(index)?;
        mem::swap(item, &mut value);
        Some(value)
    }

    pub fn get_slice(&self, a: usize, b: usize) -> Option<List> {
        let items = self.items.borrow();
        let v = items.get(a..b)?.to_vec();
        Some(List {
            items: Rc::new(RefCell::new(v)),
        })
    }

    // pub fn set_slice(
    //     &self,
    //     src: &List,
    //     src_offset: usize,
    //     offset: usize,
    //     len: usize,
    // ) -> Option<()> {
    //     let mut items = self.items.borrow_mut();
    //     if Rc::ptr_eq(&self.items, &src.items) {
    //         if len + src_offset > items.len() || len + offset > items.len() {
    //             return None;
    //         }
    //         for i in 0..len {
    //             items[offset + i] = items[src_offset + i].clone();
    //         }
    //     } else {
    //         let dst = items.get_mut(offset..len + offset)?;
    //         dst.clone_from_slice(src.items.borrow().get(src_offset..len + src_offset)?);
    //     }
    //     Some(())
    // }

    pub fn push(&self, val: Value) {
        self.items.borrow_mut().push(val);
    }

    pub fn pop(&self) -> Option<Value> {
        self.items.borrow_mut().pop()
    }

    pub fn append(&self, mut t: Vec<Value>) {
        self.items.borrow_mut().append(&mut t);
    }
}

impl Identity for List {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items) as usize
    }
}
