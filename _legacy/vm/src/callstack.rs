use std::mem::swap;

use crate::bytecode::OpError;
use crate::datamodel::Value;

pub struct CallStack {
    stack: Vec<Value>,
    locals: Vec<Value>,
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            stack: Vec::new(),
            locals: Vec::new(),
        }
    }

    pub fn load(&self, index: u8) -> Result<&Value, OpError> {
        self.locals
            .get(index as usize)
            .ok_or(OpError::LocalRead(index))
    }

    fn get_mut_or_resize(&mut self, index: u8) -> &mut Value {
        let index = index as usize;
        if index >= self.locals.len() {
            self.locals.resize_with(index + 1, || Value::None);
        }
        unsafe { self.locals.get_unchecked_mut(index) }
    }

    pub fn store(&mut self, index: u8, val: Value) {
        let out = self.get_mut_or_resize(index);
        *out = val;
    }

    pub fn swap(&mut self, index: u8, val: &mut Value) {
        let out = self.get_mut_or_resize(index);
        swap(out, val);
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Result<Value, OpError> {
        self.stack.pop().ok_or(OpError::StackEmpty)
    }
}
