use std::convert::TryInto;

use crate::datamodel::{Integer, Real, Value};

use super::{CallStack, OpAction, OpError, Operation};

macro_rules! impl_math_op {
    ($name:ident, $e:expr) => {
        new_op_empty!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                let rhs = m.pop()?;
                let lhs = m.pop()?;
                let result = match lhs {
                    Value::Integer(lhs) => {
                        let rhs: Integer = rhs.try_into()?;
                        $e(lhs, rhs).into()
                    }
                    Value::Real(lhs) => {
                        let rhs: Real = rhs.try_into()?;
                        $e(lhs, rhs).into()
                    }
                    _ => return Err(OpError::BadType(lhs.get_type())),
                };
                m.push(result);
                Ok(OpAction::None)
            }
        }
    };
}

impl_math_op!(Add, |lhs, rhs| lhs + rhs);
impl_math_op!(Sub, |lhs, rhs| lhs - rhs);
impl_math_op!(Mul, |lhs, rhs| lhs * rhs);
impl_math_op!(Div, |lhs, rhs| lhs / rhs);
impl_math_op!(Rem, |lhs, rhs| lhs % rhs);

new_op_empty!(Neg);
impl Operation for Neg {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = m.pop()?;
        let val = match val {
            Value::Integer(val) => (-val).into(),
            Value::Real(val) => (-val).into(),
            _ => return Err(OpError::BadType(val.get_type())),
        };
        m.push(val);
        Ok(OpAction::None)
    }
}
