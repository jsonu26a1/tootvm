use std::convert::TryInto;

use crate::datamodel::Integer;

use super::{CallStack, OpAction, OpError, Operation};

macro_rules! impl_int_op {
    ($name:ident, $e:expr) => {
        new_op_empty!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                let rhs: Integer = m.pop()?.try_into()?;
                let lhs: Integer = m.pop()?.try_into()?;
                let result = $e(lhs, rhs).into();
                m.push(result);
                Ok(OpAction::None)
            }
        }
    };
}

impl_int_op!(Shl, |lhs, rhs| lhs << rhs);
impl_int_op!(Shr, |lhs, rhs| lhs >> rhs);
impl_int_op!(And, |lhs, rhs| lhs & rhs);
impl_int_op!(Or, |lhs, rhs| lhs | rhs);
impl_int_op!(Xor, |lhs, rhs| lhs ^ rhs);

new_op_empty!(Not);
impl Operation for Not {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: Integer = m.pop()?.try_into()?;
        m.push((!val).into());
        Ok(OpAction::None)
    }
}
