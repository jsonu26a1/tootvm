use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::{Identity, Value};

#[derive(Clone)]
pub struct Tuple {
    items: Rc<[RefCell<Value>]>,
}

impl Tuple {
    pub fn new(items: Vec<RefCell<Value>>) -> Tuple {
        Tuple {
            items: Rc::from(items),
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = Value>) -> Tuple {
        let items = iter.map(|v| RefCell::new(v)).collect();
        Tuple::new(items)
    }

    pub fn empty(n: usize) -> Tuple {
        let mut v = Vec::with_capacity(n);
        v.resize_with(n, || RefCell::new(Value::None));
        Tuple::new(v)
    }

    pub fn iter(&self) -> impl Iterator<Item = Value> + '_ {
        self.items.iter().map(|r| r.borrow().clone())
    }

    pub fn downgrade(&self) -> TupleWeak {
        TupleWeak {
            weakref: Rc::downgrade(&self.items),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        Some(self.items.get(index)?.borrow().clone())
    }

    pub fn set(&self, index: usize, value: Value) -> Option<Value> {
        Some(self.items.get(index)?.replace(value))
    }
}

impl Identity for Tuple {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.items).cast::<RefCell<Value>>() as usize
    }
}

#[derive(Clone)]
pub struct TupleWeak {
    weakref: Weak<[RefCell<Value>]>,
}

impl TupleWeak {
    pub fn upgrade(&self) -> Option<Tuple> {
        Some(Tuple {
            items: self.weakref.upgrade()?,
        })
    }
}

// Weak::as_ptr not implemented for [T] since it is unsized
// see https://users.rust-lang.org/t/why-is-rc-as-ptr-impl-for-sized-but-not-weak-as-ptr/45059
// impl Identity for TupleWeak {
//     fn identity(&self) -> usize {
//         Weak::as_ptr(&self.weakref) as usize
//     }
// }

impl Identity for TupleWeak {
    fn identity(&self) -> usize {
        match self.weakref.upgrade() {
            Some(strong) => Rc::as_ptr(&strong).cast::<RefCell<Value>>() as usize,
            None => 0,
        }
    }
}
