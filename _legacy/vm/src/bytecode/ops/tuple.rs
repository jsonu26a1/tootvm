use std::cell::RefCell;
use std::convert::TryInto;

use crate::datamodel::{List, Tuple, TupleWeak};

use super::{CallStack, OpAction, OpError, Operation};

new_op! {
    pub struct TupleCreate {
        items: u8,
    }
}

impl Operation for TupleCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let mut acc = Vec::new();
        for _ in 0..self.items {
            let item = m.pop()?;
            acc.push(RefCell::new(item.clone()));
        }
        m.push(Tuple::new(acc).into());
        Ok(OpAction::None)
    }
}

new_op_empty!(TupleFromList);
impl Operation for TupleFromList {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let list: List = m.pop()?.try_into()?;
        let tuple = Tuple::from_iter(list.as_slice().iter().map(|v| v.clone()));
        m.push(tuple.into());
        Ok(OpAction::None)
    }
}

new_op_empty!(TupleWeakRef);
impl Operation for TupleWeakRef {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let tuple: Tuple = m.pop()?.try_into()?;
        let weak = tuple.downgrade();
        m.push(weak.into());
        Ok(OpAction::None)
    }
}

new_op_empty!(TupleWeakUpgrade);
impl Operation for TupleWeakUpgrade {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let weak: TupleWeak = m.pop()?.try_into()?;
        let tuple = weak.upgrade();
        m.push(tuple.into());
        Ok(OpAction::None)
    }
}
