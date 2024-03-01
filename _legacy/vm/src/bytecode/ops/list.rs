use std::convert::TryInto;

use crate::datamodel::{List, Value};

use super::{CallStack, OpAction, OpError, Operation};

new_op! {
    pub struct ListCreate {
        items: u8,
    }
}

impl Operation for ListCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for _ in 0..self.items {
            let item = m.pop()?;
            acc.push(item.clone());
        }
        m.push(List::new(acc).into());
        Ok(OpAction::None)
    }
}

new_op_empty!(ListPush);
impl Operation for ListPush {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let val: Value = m.pop()?;
        let list: List = m.pop()?.try_into()?;
        list.push(val);
        Ok(OpAction::None)
    }
}

new_op_empty!(ListPop);
impl Operation for ListPop {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let list: List = m.pop()?.try_into()?;
        let val = match list.pop() {
            Some(val) => val,
            None => return Err(OpError::IndexRead(0)),
        };
        m.push(val);
        Ok(OpAction::None)
    }
}

new_op_empty!(ListGetSlice);
impl Operation for ListGetSlice {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let b: i64 = m.pop()?.try_into()?;
        let a: i64 = m.pop()?.try_into()?;
        let list: List = m.pop()?.try_into()?;
        let slice = list
            .get_slice(a as usize, b as usize)
            .ok_or(OpError::IndexRead(b))?;
        m.push(slice.into());
        Ok(OpAction::None)
    }
}
