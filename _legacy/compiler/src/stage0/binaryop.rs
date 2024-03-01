use super::{ops, CodeGenerator, Expr};

pub struct BinaryOp {
    pub op_type: BinaryOpType,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[rustfmt::skip]
pub enum BinaryOpType {
    Add, Sub, Mul, Div, Rem, Shl, Shr, And, Or, Xor,
    Equal, NotEqual, Greater, GreaterOrEqual, Less, LessOrEqual,
    Identity, LogicAnd, LogicOr
}

impl BinaryOp {
    pub fn compile(&self, g: &mut CodeGenerator) {
        match self.op_type {
            BinaryOpType::Add => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Add.into());
            }
            BinaryOpType::Sub => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Sub.into());
            }
            BinaryOpType::Mul => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Mul.into());
            }
            BinaryOpType::Div => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Div.into());
            }
            BinaryOpType::Rem => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Rem.into());
            }
            BinaryOpType::Shl => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Shl.into());
            }
            BinaryOpType::Shr => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Shr.into());
            }
            BinaryOpType::And => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::And.into());
            }
            BinaryOpType::Or => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Or.into());
            }
            BinaryOpType::Xor => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Xor.into());
            }
            BinaryOpType::Identity => {
                self.lhs.compile(g);
                self.rhs.compile(g);
                g.push(ops::Cmp.into());
            }
            BinaryOpType::Equal => {
                let label_true = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // if 0, jump to label_true
                g.push_jump(label_true, ops::JumpZero::new(0).into());
                // push false (0)
                g.push(ops::LiteralCreate::new(0.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push true (1)
                g.label_here(label_true);
                g.push(ops::LiteralCreate::new(1.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::NotEqual => {
                let label_false = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // if 0, jump to label_false
                g.push_jump(label_false, ops::JumpZero::new(0).into());
                // push true (1)
                g.push(ops::LiteralCreate::new(1.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push false (0)
                g.label_here(label_false);
                g.push(ops::LiteralCreate::new(0.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::Greater => {
                let label_true = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // flip sign
                g.push(ops::Neg.into());
                // if negative, jump to label_true
                g.push_jump(label_true, ops::JumpNeg::new(0).into());
                // push false (0)
                g.push(ops::LiteralCreate::new(0.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push true (1)
                g.label_here(label_true);
                g.push(ops::LiteralCreate::new(1.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::GreaterOrEqual => {
                let label_false = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // if negative, jump to label_false
                g.push_jump(label_false, ops::JumpNeg::new(0).into());
                // push true (1)
                g.push(ops::LiteralCreate::new(1.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push false (0)
                g.label_here(label_false);
                g.push(ops::LiteralCreate::new(0.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::Less => {
                let label_true = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // if negative, jump to label_true
                g.push_jump(label_true, ops::JumpNeg::new(0).into());
                // push false (0)
                g.push(ops::LiteralCreate::new(0.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push true (1)
                g.label_here(label_true);
                g.push(ops::LiteralCreate::new(1.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::LessOrEqual => {
                let label_false = g.create_label();
                let label_next = g.create_label();
                // compile lhs and rhs
                self.lhs.compile(g);
                self.rhs.compile(g);
                // push -1 if a < b, 0 if a == b, 1 if a > b
                g.push(ops::Cmp.into());
                // flip sign
                g.push(ops::Neg.into());
                // if negative, jump to label_false
                g.push_jump(label_false, ops::JumpNeg::new(0).into());
                // push true (1)
                g.push(ops::LiteralCreate::new(1.into()).into());
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push false (0)
                g.label_here(label_false);
                g.push(ops::LiteralCreate::new(0.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::LogicAnd => {
                let label_false = g.create_label();
                let label_next = g.create_label();
                // compile lhs
                self.lhs.compile(g);
                // if zero, jump to label_false
                g.push_jump(label_false, ops::JumpZero::new(0).into());
                // compile rhs
                self.rhs.compile(g);
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push false (0)
                g.label_here(label_false);
                g.push(ops::LiteralCreate::new(0.into()).into());
                g.label_here(label_next);
            }
            BinaryOpType::LogicOr => {
                let label_true = g.create_label();
                let label_next = g.create_label();
                // compile lhs
                self.lhs.compile(g);
                // flip sign
                g.push(ops::Neg.into());
                // if negative, jump to label_true
                g.push_jump(label_true, ops::JumpNeg::new(0).into());
                // compile rhs
                self.rhs.compile(g);
                // jump to label_next
                g.push_jump(label_next, ops::Jump::new(0).into());
                // push true (1)
                g.label_here(label_true);
                g.push(ops::LiteralCreate::new(1.into()).into());
                g.label_here(label_next);
            }
        }
    }
}
