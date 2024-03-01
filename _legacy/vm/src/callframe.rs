use crate::bytecode::{OpAction, OpError, Operation};
use crate::datamodel::{Function, Value};

use super::CallStack;

pub struct CallFrame {
    pub parent: Option<Box<CallFrame>>,
    pub function: Function,
    pub cursor: usize,
    pub stack: CallStack,
}

impl CallFrame {
    pub fn new(function: Function) -> CallFrame {
        let mut stack = CallStack::new();
        stack.store(0, function.module.clone().into());
        CallFrame {
            parent: None,
            function,
            cursor: 0,
            stack,
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn jump(&mut self, index: i32) {
        self.cursor = (self.cursor as isize + index as isize) as usize;
    }

    pub fn exec(&mut self) -> Result<OpAction, OpError> {
        let op = match self.function.ops.get(self.cursor) {
            Some(op) => op.clone(),
            None => return Ok(OpAction::Return(Value::None)),
        };
        self.cursor += 1;
        op.exec(&mut self.stack)
    }
}
