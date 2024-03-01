use std::convert::TryInto;

use crate::datamodel::{Integer, List, Table, Tuple, Value};

use super::{CallStack, OpAction, OpError, Operation};

new_op_empty!(TableCreate);
impl Operation for TableCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let list: List = m.pop()?.try_into()?;
        let mut table = Vec::new();
        for val in list.as_slice().iter() {
            let tuple: &Tuple = val.try_into()?;
            let k: Integer = tuple.get(0).ok_or(OpError::IndexRead(0))?.try_into()?;
            let v: Value = tuple.get(1).ok_or(OpError::IndexRead(1))?;
            table.push((k as u64, v));
        }
        let table = Table::new(table);
        m.push(table.into());
        Ok(OpAction::None)
    }
}
