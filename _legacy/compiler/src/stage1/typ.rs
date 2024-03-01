use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum Type {
    Parameter(usize),
    Alias(usize),
    GenericAlias(usize, Rc<[Type]>),
    Option(Box<Type>),
    Weak(Box<Type>),
    Bool,
    Integer,
    Real,
    Tuple(Rc<[Type]>),
    Table,
    List(Box<Type>),
    Buffer,
    Function(Rc<FunctionType>),
    NativeFn(Rc<FunctionType>),
    Unknown,
}

impl Type {
    pub fn is_concrete(&self) -> bool {
        match self {
            Type::Parameter(_) => false,
            Type::GenericAlias(_, items) => {
                for item in items.iter() {
                    if !item.is_concrete() {
                        return false;
                    }
                }
                true
            }
            Type::Option(t) => t.is_concrete(),
            Type::Weak(t) => t.is_concrete(),
            Type::Tuple(items) => {
                for item in items.iter() {
                    if !item.is_concrete() {
                        return false;
                    }
                }
                true
            }
            Type::List(t) => t.is_concrete(),
            Type::Function(f) => f.is_concrete(),
            Type::NativeFn(f) => f.is_concrete(),
            _ => true,
        }
    }

    pub fn resolve_params(&self, params: &[Type]) -> Option<Type> {
        match self {
            Type::Parameter(i) => params.get(*i).cloned(),
            Type::GenericAlias(i, items) => {
                let mut v = Vec::new();
                for item in items.iter() {
                    v.push(item.resolve_params(params)?);
                }
                Some(Type::GenericAlias(*i, Rc::from(v)))
            },
            Type::Option(t) => Some(Type::Option(Box::new(t.resolve_params(params)?))),
            Type::Weak(t) => Some(Type::Weak(Box::new(t.resolve_params(params)?))),
            Type::Tuple(items) => {
                let mut v = Vec::new();
                for item in items.iter() {
                    v.push(item.resolve_params(params)?);
                }
                Some(Type::Tuple(Rc::from(v)))
            },
            Type::List(t) => Some(Type::List(Box::new(t.resolve_params(params)?))),
            Type::Function(f) => Some(Type::Function(Rc::new(f.resolve_params(params)?))),
            Type::NativeFn(f) => Some(Type::NativeFn(Rc::new(f.resolve_params(params)?))),
            _ => Some(self.clone())
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionType {
    pub args: Vec<Type>,
    pub ret: Type,
}

impl FunctionType {
    pub fn is_concrete(&self) -> bool {
        for arg in &self.args {
            if !arg.is_concrete() {
                return false;
            }
        }
        self.ret.is_concrete()
    }

    pub fn resolve_params(&self, params: &[Type]) -> Option<FunctionType> {
        let mut args = Vec::new();
        for arg in &self.args {
            args.push(arg.resolve_params(params)?);
        }
        let ret = self.ret.resolve_params(params)?;
        Some(FunctionType { args, ret })
    }
}
