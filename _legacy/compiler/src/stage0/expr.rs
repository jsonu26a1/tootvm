use super::{ops, ops::LiteralValue, BinaryOp, CodeGenerator, UnaryOp};

pub type Var = usize;

#[derive(Clone, Copy)]
pub struct Span<T> {
    // TODO info for source map
    pub span: (),
    pub inner: T,
}

pub enum Expr {
    LiteralValue(Span<LiteralValue>),
    Var(Span<Var>),
    ModuleRef,
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    SeqIndex {
        seq: Box<Expr>,
        index: Box<Expr>,
    },
    SeqLen {
        seq: Box<Expr>,
    },
    SeqToList {
        seq: Box<Expr>,
    },
    TupleCreate(Vec<Expr>),
    TupleFromList(Box<Expr>),
    TupleWeakRef(Box<Expr>),
    TupleWeakUpgrade(Box<Expr>),
    TableCreate(Box<Expr>),
    ListCreate(Vec<Expr>),
    ListGetSlice {
        list: Box<Expr>,
        a: Box<Expr>,
        b: Box<Expr>,
    },
    ListPop(Box<Expr>),
    BufferCreate(Box<Expr>),
    BufferGetSlice {
        buffer: Box<Expr>,
        a: Box<Expr>,
        b: Box<Expr>,
    },
}

impl Expr {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            Expr::LiteralValue(l) => g.push(ops::LiteralCreate::new(l.inner).into()),
            Expr::Var(var) => g.push_var_load(var.inner),
            Expr::ModuleRef => g.push(ops::StackLoad::new(0).into()),
            Expr::BinaryOp(b) => b.compile(g),
            Expr::UnaryOp(u) => u.compile(g),
            Expr::Call { func, args } => {
                func.compile(g);
                assert!(args.len() <= 255);
                for arg in args {
                    arg.compile(g);
                }
                g.push(ops::Call::new(args.len() as u8).into());
            }
            Expr::SeqIndex { seq, index } => {
                seq.compile(g);
                index.compile(g);
                g.push(ops::SeqGet.into());
            }
            Expr::SeqLen { seq } => {
                seq.compile(g);
                g.push(ops::SeqLen.into());
            }
            Expr::SeqToList { seq } => {
                seq.compile(g);
                g.push(ops::SeqToList.into());
            }
            Expr::TupleCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    item.compile(g);
                }
                g.push(ops::TupleCreate::new(items.len() as u8).into());
            }
            Expr::TupleFromList(e) => {
                e.compile(g);
                g.push(ops::TupleFromList.into());
            }
            Expr::TupleWeakRef(e) => {
                e.compile(g);
                g.push(ops::TupleWeakRef.into());
            }
            Expr::TupleWeakUpgrade(e) => {
                e.compile(g);
                g.push(ops::TupleWeakUpgrade.into());
            }
            Expr::TableCreate(e) => {
                e.compile(g);
                g.push(ops::TableCreate.into());
            }
            Expr::ListCreate(items) => {
                assert!(items.len() <= 255);
                for item in items {
                    item.compile(g);
                }
                g.push(ops::ListCreate::new(items.len() as u8).into());
            }
            Expr::ListGetSlice { list, a, b } => {
                list.compile(g);
                a.compile(g);
                b.compile(g);
                g.push(ops::ListGetSlice.into());
            }
            Expr::ListPop(e) => {
                e.compile(g);
                g.push(ops::ListPop.into());
            }
            Expr::BufferCreate(e) => {
                e.compile(g);
                g.push(ops::BufferCreate.into());
            }
            Expr::BufferGetSlice { buffer, a, b } => {
                buffer.compile(g);
                a.compile(g);
                b.compile(g);
                g.push(ops::BufferGetSlice.into());
            }
        }
    }

    pub fn find_vars(&self) -> Vec<Span<Var>> {
        let mut vars = Vec::new();
        self.acc_vars(&mut vars);
        vars
    }

    fn acc_vars(&self, vars: &mut Vec<Span<Var>>) {
        match self {
            Expr::LiteralValue(_) => {}
            Expr::Var(var) => vars.push(*var),
            Expr::ModuleRef => {}
            Expr::BinaryOp(b) => {
                b.lhs.acc_vars(vars);
                b.rhs.acc_vars(vars);
            }
            Expr::UnaryOp(u) => u.expr.acc_vars(vars),
            Expr::Call { func, args } => {
                func.acc_vars(vars);
                for arg in args {
                    arg.acc_vars(vars);
                }
            }
            Expr::SeqIndex { seq, index } => {
                seq.acc_vars(vars);
                index.acc_vars(vars);
            }
            Expr::SeqLen { seq } => seq.acc_vars(vars),
            Expr::SeqToList { seq } => seq.acc_vars(vars),
            Expr::TupleCreate(exprs) => {
                for e in exprs {
                    e.acc_vars(vars);
                }
            }
            Expr::TupleFromList(e) => e.acc_vars(vars),
            Expr::TupleWeakRef(e) => e.acc_vars(vars),
            Expr::TupleWeakUpgrade(e) => e.acc_vars(vars),
            Expr::TableCreate(e) => e.acc_vars(vars),
            Expr::ListCreate(exprs) => {
                for e in exprs {
                    e.acc_vars(vars);
                }
            }
            Expr::ListGetSlice { list, a, b } => {
                list.acc_vars(vars);
                a.acc_vars(vars);
                b.acc_vars(vars);
            }
            Expr::ListPop(e) => e.acc_vars(vars),
            Expr::BufferCreate(e) => e.acc_vars(vars),
            Expr::BufferGetSlice { buffer, a, b } => {
                buffer.acc_vars(vars);
                a.acc_vars(vars);
                b.acc_vars(vars);
            }
        }
    }
}
