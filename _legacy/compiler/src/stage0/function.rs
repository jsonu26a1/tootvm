use std::collections::BTreeSet;

use super::{bytecode, CodeGenerator, Expr, If, Statement, Var};

pub struct Function {
    pub args: Vec<Var>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn compile(mut self) -> bytecode::Function {
        let mut setup = Vec::new();
        for &arg in &self.args {
            setup.push(Statement::BindVar(arg));
            setup.push(Statement::InitVar(arg));
        }
        setup.append(&mut self.body);
        self.body = setup;
        if let Err(unknown_scope_vars) = self.block_scope_analysis() {
            panic!(
                "found {} variables with unknown scope",
                unknown_scope_vars.len()
            );
        }
        let mut g = CodeGenerator::new();
        for statement in &self.body {
            statement.compile(&mut g);
        }
        bytecode::Function { ops: g.into_vec() }
    }

    fn block_scope_analysis(&mut self) -> Result<(), Vec<Var>> {
        let mut seen = BTreeSet::new();
        let mut b = BlockScopeAnalysis::new(&mut seen);
        let unknown_scope_vars = b.process_block(&mut self.body);
        if unknown_scope_vars.is_empty() {
            Ok(())
        } else {
            Err(unknown_scope_vars)
        }
    }
}

struct DeferredDrop {
    loc: usize,
    var: Var,
}

struct BlockScopeAnalysis<'a> {
    seen: &'a mut BTreeSet<Var>,
    bindings: BTreeSet<Var>,
    drops: Vec<DeferredDrop>,
    loc: usize,
}

impl<'a> BlockScopeAnalysis<'a> {
    fn new(seen: &'a mut BTreeSet<Var>) -> Self {
        BlockScopeAnalysis {
            seen,
            bindings: BTreeSet::new(),
            drops: Vec::new(),
            loc: 0,
        }
    }

    fn process_block(&mut self, block: &mut Vec<Statement>) -> Vec<Var> {
        for (loc, statement) in block.iter_mut().enumerate().rev() {
            self.loc = loc;
            self.process_statement(statement);
        }
        let mut parent_scope = Vec::new();
        for drop in self.drops.iter().rev() {
            if self.bindings.contains(&drop.var) {
                block.insert(drop.loc + 1, Statement::DropVar(drop.var));
            } else {
                parent_scope.push(drop.var);
            }
        }
        parent_scope
    }

    fn process_var(&mut self, var: Var) {
        let dropped = self.seen.insert(var);
        if dropped {
            self.drops.push(DeferredDrop { loc: self.loc, var });
        }
    }

    fn process_expr(&mut self, expr: &Expr) {
        for var in expr.find_vars() {
            self.process_var(var.inner);
        }
    }

    fn process_child_block(&mut self, block: &mut Vec<Statement>) {
        let mut b = BlockScopeAnalysis::new(&mut self.seen);
        let outer_scope_vars = b.process_block(block);
        for var in outer_scope_vars {
            self.drops.push(DeferredDrop { loc: self.loc, var });
        }
    }

    fn process_if(&mut self, if_: &mut If) {
        self.process_expr(&if_.condition);
        self.process_child_block(&mut if_.body);
    }

    fn process_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::BindVar(i) => {
                self.bindings.insert(*i);
            }
            Statement::DropVar(_) => {
                panic!("unexpected DropVar statement during block scope analysis");
            }
            Statement::InitVar(i) => self.process_var(*i),
            Statement::Loop(l) => {
                if let Some(condition) = l.condition.as_ref() {
                    self.process_expr(condition);
                }
                self.process_child_block(&mut l.body);
            }
            Statement::Break { .. } => {}
            Statement::Continue { .. } => {}
            Statement::Expr(expr) => self.process_expr(expr),
            Statement::Return(expr) => self.process_expr(expr),
            Statement::IfElse(s) => {
                self.process_if(&mut s.if_);
                for if_ in s.else_if.iter_mut() {
                    self.process_if(if_);
                }
                self.process_child_block(&mut s.else_);
            }
            Statement::Assign { place, value } => {
                self.process_expr(place);
                self.process_expr(value);
            }
            Statement::SeqAppend { seq, src } => {
                self.process_expr(seq);
                self.process_expr(src);
            }
            Statement::SeqResize { seq, len } => {
                self.process_expr(seq);
                self.process_expr(len);
            }
            Statement::ListPush { list, value } => {
                self.process_expr(list);
                self.process_expr(value);
            }
            Statement::BufferSetSlice {
                buffer,
                src,
                src_offset,
                offset,
                len,
            } => {
                self.process_expr(buffer);
                self.process_expr(src);
                self.process_expr(src_offset);
                self.process_expr(offset);
                self.process_expr(len);
            }
        }
    }
}
