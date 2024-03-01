use super::{ops::LiteralValue, Function};

pub enum ModuleItem {
    LiteralValue(LiteralValue),
    Buffer(Vec<u8>),
    ModuleRef(u32),
    Function(Function),
}

pub struct Module {
    pub items: Vec<ModuleItem>,
}

pub struct Program {
    pub modules: Vec<Module>,
}
