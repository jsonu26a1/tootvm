use std::convert::TryInto;

use crate::datamodel::{Integer, Real};

use super::{CallStack, OpAction, OpError, Operation};

macro_rules! impl_real_op {
    ($name:ident, $e:expr) => {
        new_op_empty!($name);
        impl Operation for $name {
            fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
                let val: Real = m.pop()?.try_into()?;
                m.push($e(val).into());
                Ok(OpAction::None)
            }
        }
    };
}

impl_real_op!(Floor, |val: f64| val.floor());
impl_real_op!(Ceil, |val: f64| val.ceil());
impl_real_op!(Trunc, |val: f64| val.trunc());
impl_real_op!(Round, |val: f64| val.round());

new_op_empty!(IntToReal);
impl Operation for IntToReal {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let int: Integer = m.pop()?.try_into()?;
        m.push((int as Real).into());
        Ok(OpAction::None)
    }
}
