use super::{BytesIO, BytesReadError, Op};

pub struct Function {
    pub ops: Vec<Op>,
}

impl BytesIO for Function {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, ops) = <Vec<Op> as BytesIO>::read(b)?;
        let f = Function { ops };
        Ok((b, f))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        <Vec<Op> as BytesIO>::write(&t.ops, b)
    }
}
