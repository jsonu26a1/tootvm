use super::{BytesIO, BytesReadError, Module};

use crate::datamodel::{Tuple, Value};

pub struct Program {
    pub modules: Vec<Module>,
}

impl Program {
    pub fn into_tuple(self) -> Tuple {
        let len = self.modules.len();
        let tuple = Tuple::empty(len);
        let mut refs = Vec::new();
        for (i, module) in self.modules.into_iter().enumerate() {
            let (module, mrefs) = module.into_tuple();
            refs.push((module.clone(), mrefs));
            tuple.set(i, module.into());
        }
        for (module, mrefs) in refs {
            for (l, r) in mrefs {
                module.set(l, tuple.get(r).unwrap_or(Value::None));
            }
        }
        tuple
    }
}

impl BytesIO for Program {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, modules) = <Vec<Module> as BytesIO>::read(b)?;
        Ok((b, Program { modules }))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        <Vec<Module> as BytesIO>::write(&t.modules, b)
    }
}
