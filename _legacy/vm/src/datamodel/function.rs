use std::rc::Rc;

use crate::bytecode::Op;

use super::{Identity, Tuple};

#[derive(Clone)]
pub struct Function {
    pub module: Tuple,
    pub ops: Rc<[Op]>,
}

impl Function {
    pub fn new(module: Tuple, ops: Vec<Op>) -> Function {
        Function {
            module,
            ops: Rc::from(ops),
        }
    }
}

impl Identity for Function {
    fn identity(&self) -> usize {
        Rc::as_ptr(&self.ops).cast::<Op>() as usize
    }
}
