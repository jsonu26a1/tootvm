use super::ops::LiteralValue;
use super::{BytesIO, BytesReadError, Function};

use crate::datamodel::{Buffer, Function as FuncVal, Tuple, Value};

pub struct Module {
    pub items: Vec<ModuleItem>,
}

impl Module {
    pub fn into_tuple(self) -> (Tuple, Vec<(usize, usize)>) {
        let len = self.items.len();
        let tuple = Tuple::empty(len);
        let mut refs = Vec::new();
        for (i, item) in self.items.into_iter().enumerate() {
            let val = match item {
                ModuleItem::LiteralValue(t) => t.into_val(),
                ModuleItem::Buffer(v) => Buffer::new(v).into(),
                ModuleItem::ModuleRef(r) => {
                    refs.push((i, r as usize));
                    Value::None
                }
                ModuleItem::Function(f) => FuncVal::new(tuple.clone(), f.ops).into(),
            };
            tuple.set(i, val);
        }
        (tuple, refs)
    }
}

impl BytesIO for Module {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, items) = <Vec<ModuleItem> as BytesIO>::read(b)?;
        Ok((b, Module { items }))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        <Vec<ModuleItem> as BytesIO>::write(&t.items, b)
    }
}

pub enum ModuleItem {
    LiteralValue(LiteralValue),
    Buffer(Vec<u8>),
    ModuleRef(u32),
    Function(Function),
}

impl BytesIO for ModuleItem {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b2, n) = <u8 as BytesIO>::read(b)?;
        match n {
            0 => {
                let (b, t) = <LiteralValue as BytesIO>::read(b2)?;
                Ok((b, ModuleItem::LiteralValue(t)))
            }
            1 => {
                let (b, n) = <u32 as BytesIO>::read(b2)?;
                let b1 = b.get(n as usize..).ok_or(BytesReadError::EndOfFile)?;
                let b2 = unsafe { b.get_unchecked(..n as usize) };
                Ok((b1, ModuleItem::Buffer(b2.to_vec())))
            }
            2 => {
                let (b, t) = <u32 as BytesIO>::read(b2)?;
                Ok((b, ModuleItem::ModuleRef(t)))
            }
            3 => {
                let (b, t) = <Function as BytesIO>::read(b2)?;
                Ok((b, ModuleItem::Function(t)))
            }
            _ => Err(BytesReadError::InvalidValue(b)),
        }
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        match t {
            ModuleItem::LiteralValue(t) => {
                let b = <u8 as BytesIO>::write(&0, b)?;
                <LiteralValue as BytesIO>::write(t, b)
            }
            ModuleItem::Buffer(t) => {
                let b = <u8 as BytesIO>::write(&1, b)?;
                let n = t.len();
                let b = <u32 as BytesIO>::write(&(n as u32), b)?;
                b.get_mut(..n)?.copy_from_slice(&*t);
                Some(unsafe { b.get_unchecked_mut(n..) })
            }
            ModuleItem::ModuleRef(t) => {
                let b = <u8 as BytesIO>::write(&2, b)?;
                <u32 as BytesIO>::write(t, b)
            }
            ModuleItem::Function(t) => {
                let b = <u8 as BytesIO>::write(&3, b)?;
                <Function as BytesIO>::write(t, b)
            }
        }
    }
}

/*
quick note: if we want dynamic loading of libraries, would that screw up programs
which use dynamic dispatch and Tables? it does have the potential to, but rather
than hardcoding the ID of each interface/trait, we could have programs fetch the
ID from the library after it is loaded.

how do want to handle dynamic loading? well, modules are just Tuples, so we don't
need to implement that here, it will be handled at runtime, not at load time.
*/
