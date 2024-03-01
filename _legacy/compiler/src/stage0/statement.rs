use super::{ops, CodeGenerator, Expr, Label, Var};

pub enum Statement {
    BindVar(Var),
    DropVar(Var),
    InitVar(Var),
    Loop(Loop),
    Break {
        label: Option<usize>,
    },
    Continue {
        label: Option<usize>,
    },
    Expr(Expr),
    Return(Expr),
    IfElse(IfElse),
    Assign {
        place: Box<Expr>,
        value: Box<Expr>,
    },
    SeqAppend {
        seq: Box<Expr>,
        src: Box<Expr>,
    },
    SeqResize {
        seq: Box<Expr>,
        len: Box<Expr>,
    },
    ListPush {
        list: Box<Expr>,
        value: Box<Expr>,
    },
    BufferSetSlice {
        buffer: Box<Expr>,
        src: Box<Expr>,
        src_offset: Box<Expr>,
        offset: Box<Expr>,
        len: Box<Expr>,
    },
}

impl Statement {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self {
            Statement::BindVar(var) => g.bind_var(*var),
            Statement::DropVar(var) => g.drop_var(*var),
            Statement::InitVar(var) => g.push_var_store(*var),
            Statement::Loop(l) => l.compile(g),
            Statement::Break { label } => {
                let label = g.loop_get_break(*label);
                g.push_jump(label, ops::Jump::new(0).into());
            }
            Statement::Continue { label } => {
                let label = g.loop_get_continue(*label);
                g.push_jump(label, ops::Jump::new(0).into());
            }
            Statement::Expr(e) => {
                e.compile(g);
                g.push(ops::StackPop.into());
            }
            Statement::Return(e) => {
                e.compile(g);
                g.push(ops::Return.into());
            }
            Statement::IfElse(s) => s.compile(g),
            Statement::Assign { place, value } => match &**place {
                Expr::Var(var) => {
                    value.compile(g);
                    g.push_var_store(var.inner);
                }
                Expr::SeqIndex { seq, index } => {
                    seq.compile(g);
                    index.compile(g);
                    value.compile(g);
                    g.push(ops::SeqSet.into());
                }
                _ => panic!("invalid place expression"),
            },
            Statement::SeqAppend { seq, src } => {
                seq.compile(g);
                src.compile(g);
                g.push(ops::SeqAppend.into());
            }
            Statement::SeqResize { seq, len } => {
                seq.compile(g);
                len.compile(g);
                g.push(ops::SeqResize.into());
            }
            Statement::ListPush { list, value } => {
                list.compile(g);
                value.compile(g);
                g.push(ops::ListPush.into());
            }
            Statement::BufferSetSlice {
                buffer,
                src,
                src_offset,
                offset,
                len,
            } => {
                buffer.compile(g);
                src.compile(g);
                src_offset.compile(g);
                offset.compile(g);
                len.compile(g);
                g.push(ops::BufferSetSlice.into());
            }
        }
    }
}

pub struct Loop {
    pub condition: Option<Expr>,
    pub label: Option<usize>,
    pub body: Vec<Statement>,
}

impl Loop {
    pub fn compile(&self, g: &mut CodeGenerator) {
        g.loop_enter(self.label);
        let label_continue = g.loop_get_continue(self.label);
        let label_break = g.loop_get_break(self.label);
        g.label_here(label_continue);
        if let Some(condition) = &self.condition {
            // compile condition
            condition.compile(g);
            // if zero, jump to label_break
            g.push_jump(label_break, ops::JumpZero::new(0).into());
        }
        for statement in &self.body {
            statement.compile(g);
        }
        // jump to label_continue
        g.push_jump(label_continue, ops::Jump::new(0).into());
        g.label_here(label_break);
        g.loop_exit(self.label);
    }
}

pub struct IfElse {
    pub if_: If,
    pub else_if: Vec<If>,
    pub else_: Vec<Statement>,
}

pub struct If {
    pub condition: Expr,
    pub body: Vec<Statement>,
}

impl IfElse {
    pub fn compile(&self, g: &mut CodeGenerator) {
        let label_endif = g.create_label();
        // compile "if" block
        self.if_.compile(g, label_endif);
        // compile "else if" blocks
        for if_ in &self.else_if {
            if_.compile(g, label_endif);
        }
        // compile "else" block
        for statement in &self.else_ {
            statement.compile(g);
        }
        g.label_here(label_endif);
    }
}

impl If {
    pub fn compile(&self, g: &mut CodeGenerator, label_endif: Label) {
        let label_next = g.create_label();
        // compile condition
        self.condition.compile(g);
        // if zero, jump to label_next
        g.push_jump(label_next, ops::JumpZero::new(0).into());
        // compile body statements
        for statement in &self.body {
            statement.compile(g);
        }
        // jump to label_endif
        g.push_jump(label_endif, ops::Jump::new(0).into());
        g.label_here(label_next);
    }
}
