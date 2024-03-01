use crate::datamodel::Value;

use super::{CallStack, OpAction, OpError, Operation};

new_op! {
    pub struct Jump {
        pub dest: i32,
    }
}

impl Operation for Jump {
    fn exec(&self, _: &mut CallStack) -> Result<OpAction, OpError> {
        Ok(OpAction::Jump(self.dest))
    }
}

new_op! {
    pub struct JumpZero {
        pub dest: i32,
    }
}

impl Operation for JumpZero {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let is_zero = match m.pop()? {
            Value::None => true,
            Value::Integer(i) => i == 0,
            Value::Real(r) => r == 0.0,
            _ => false,
        };
        if is_zero {
            Ok(OpAction::Jump(self.dest))
        } else {
            Ok(OpAction::None)
        }
    }
}

new_op! {
    pub struct JumpNeg {
        pub dest: i32,
    }
}

impl Operation for JumpNeg {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let is_zero = match m.pop()? {
            Value::Integer(i) => i < 0,
            Value::Real(r) => r < 0.0,
            _ => false,
        };
        if is_zero {
            Ok(OpAction::Jump(self.dest))
        } else {
            Ok(OpAction::None)
        }
    }
}
