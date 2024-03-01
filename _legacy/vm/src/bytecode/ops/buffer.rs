use std::convert::TryInto;

use crate::datamodel::Buffer;

use super::{CallStack, OpAction, OpError, Operation};

new_op_empty!(BufferCreate);
impl Operation for BufferCreate {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let len: i64 = m.pop()?.try_into()?;
        let val = Buffer::empty();
        val.resize(len as usize);
        m.push(val.into());
        Ok(OpAction::None)
    }
}

new_op_empty!(BufferGetSlice);
impl Operation for BufferGetSlice {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let b: i64 = m.pop()?.try_into()?;
        let a: i64 = m.pop()?.try_into()?;
        let buffer: Buffer = m.pop()?.try_into()?;
        let slice = buffer
            .get_slice(a as usize, b as usize)
            .ok_or(OpError::IndexRead(b))?;
        m.push(slice.into());
        Ok(OpAction::None)
    }
}

new_op_empty!(BufferSetSlice);
impl Operation for BufferSetSlice {
    fn exec(&self, m: &mut CallStack) -> Result<OpAction, OpError> {
        let len: i64 = m.pop()?.try_into()?;
        let offset: i64 = m.pop()?.try_into()?;
        let src_offset: i64 = m.pop()?.try_into()?;
        let src: Buffer = m.pop()?.try_into()?;
        let buffer: Buffer = m.pop()?.try_into()?;
        buffer
            .set_slice(&src, src_offset as usize, offset as usize, len as usize)
            .ok_or(OpError::IndexWrite(len))?;
        Ok(OpAction::None)
    }
}
