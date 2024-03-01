use crate::datamodel::Value;

use super::{BytesIO, BytesReadError, CallStack, OpAction, OpError, Operation};

#[derive(Clone, Copy)]
pub enum LiteralValue {
    None,
    Integer(i64),
    Real(f64),
}

impl From<i64> for LiteralValue {
    fn from(t: i64) -> LiteralValue {
        LiteralValue::Integer(t)
    }
}

impl LiteralValue {
    pub fn into_val(&self) -> Value {
        match self {
            LiteralValue::None => Value::None,
            LiteralValue::Integer(i) => Value::Integer(*i),
            LiteralValue::Real(r) => Value::Real(*r),
        }
    }
}

impl BytesIO for LiteralValue {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b2, n) = <u8 as BytesIO>::read(b)?;
        match n {
            0 => Ok((b2, LiteralValue::None)),
            1 => {
                let (b, int) = <i64 as BytesIO>::read(b2)?;
                Ok((b, LiteralValue::Integer(int)))
            }
            2 => {
                let (b, real) = <f64 as BytesIO>::read(b2)?;
                Ok((b, LiteralValue::Real(real)))
            }
            _ => Err(BytesReadError::InvalidValue(b)),
        }
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        match t {
            LiteralValue::None => Some(<u8 as BytesIO>::write(&0, b)?),
            LiteralValue::Integer(int) => Some(<(u8, i64) as BytesIO>::write(&(1, *int), b)?),
            LiteralValue::Real(real) => Some(<(u8, f64) as BytesIO>::write(&(2, *real), b)?),
        }
    }
}

new_op! {
    pub struct LiteralCreate {
        val: LiteralValue,
    }
}

impl Operation for LiteralCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val = match self.val {
            LiteralValue::None => Value::None,
            LiteralValue::Integer(int) => int.into(),
            LiteralValue::Real(real) => real.into(),
        };
        m.push(val);
        Ok(OpAction::None)
    }
}
