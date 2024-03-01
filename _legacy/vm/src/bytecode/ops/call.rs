use crate::datamodel::Value;

use super::{CallStack, OpAction, OpError, Operation};

new_op! {
    pub struct Call {
        args: u8,
    }
}

impl Operation for Call {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        // NOTE: for expr `Call(A, B, C)`, args is reversed: `[C, B, A]`
        let mut args = Vec::new();
        for _ in 0..self.args {
            let val = m.pop()?;
            args.push(val);
        }
        // this should be commented out, since we want args to be reversed
        // see crate::vm::VirtualMachine
        // let args = args.into_iter().rev().collect();
        let target = m.pop()?;
        match target {
            Value::Function(t) => Ok(OpAction::Call(t, args)),
            Value::NativeFn(t) => Ok(OpAction::CallNative(t, args)),
            _ => Err(OpError::BadType(target.get_type())),
        }
    }
}

new_op_empty!(Return);
impl Operation for Return {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let output = m.pop()?;
        Ok(OpAction::Return(output))
    }
}
