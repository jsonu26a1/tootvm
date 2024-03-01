use std::cmp::Ordering;
use std::convert::TryInto;

use crate::datamodel::{
    Buffer, Function, Identity, List, NativeFn, Table, Unknown, Value, ValueTryIntoError, ValueType,
};

use super::{CallStack, OpAction, OpError, Operation};

new_op_empty!(Cmp);
impl Operation for Cmp {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let rhs = m.pop()?;
        let lhs = m.pop()?;
        let result = match lhs {
            Value::None => match rhs {
                Value::None => Ordering::Equal.into(),
                _ => Ordering::Less.into(),
            },
            Value::Integer(lhs) => lhs.cmp(&rhs.try_into()?).into(),
            Value::Real(lhs) => lhs.partial_cmp(&rhs.try_into()?).into(),
            Value::Tuple(lhs) => cmp_tuple(lhs.identity(), rhs)?.into(),
            Value::TupleWeak(lhs) => cmp_tuple(lhs.identity(), rhs)?.into(),
            Value::Table(lhs) => {
                (lhs.identity() == TryInto::<Table>::try_into(rhs)?.identity()).into()
            }
            Value::List(lhs) => {
                (lhs.identity() == TryInto::<List>::try_into(rhs)?.identity()).into()
            }
            Value::Buffer(lhs) => {
                (lhs.identity() == TryInto::<Buffer>::try_into(rhs)?.identity()).into()
            }
            Value::Function(lhs) => {
                (lhs.identity() == TryInto::<Function>::try_into(rhs)?.identity()).into()
            }
            Value::NativeFn(lhs) => (lhs == TryInto::<NativeFn>::try_into(rhs)?).into(),
            Value::Unknown(lhs) => {
                (lhs.identity() == TryInto::<Unknown>::try_into(rhs)?.identity()).into()
            }
        };
        m.push(result);
        Ok(OpAction::None)
    }
}

fn cmp_tuple(lhs: usize, rhs: Value) -> Result<bool, ValueTryIntoError> {
    let rhs = match rhs {
        Value::Tuple(rhs) => rhs.identity(),
        Value::TupleWeak(rhs) => rhs.identity(),
        _ => {
            return Err(ValueTryIntoError {
                found: rhs.get_type(),
                expected: ValueType::Tuple,
            })
        }
    };
    Ok(lhs == rhs)
}

new_op_empty!(GetType);
impl Operation for GetType {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = m.pop()?;
        let ty = val.get_type() as i64;
        m.push(ty.into());
        Ok(OpAction::None)
    }
}
