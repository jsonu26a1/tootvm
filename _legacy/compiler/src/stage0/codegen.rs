use std::collections::BTreeMap;

use super::{bytecode::Op, ops, Var};

pub type Label = usize;

struct LabelData {
    target: Option<usize>,
    pub jumps: Vec<usize>,
}

impl LabelData {
    pub fn new() -> LabelData {
        LabelData {
            target: None,
            jumps: Vec::new(),
        }
    }

    pub fn set_target(&mut self, target: usize) {
        match self.target {
            Some(_) => panic!("label target already set"),
            None => self.target = Some(target),
        }
    }

    pub fn get_target(&self) -> usize {
        self.target.expect("label target not set")
    }
}

pub struct CodeGenerator {
    ops: Vec<Op>,
    labels: Vec<LabelData>,
    loops: BTreeMap<usize, (Label, Label)>,
    loop_stack: Vec<(Label, Label)>,
    vars: BTreeMap<Var, u8>,
    dropped: Vec<u8>,
    next_index: u8,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            ops: Vec::new(),
            labels: Vec::new(),
            loops: BTreeMap::new(),
            loop_stack: Vec::new(),
            vars: BTreeMap::new(),
            dropped: Vec::new(),
            // next_index starts at 1, because module ref is at index 0
            next_index: 1,
        }
    }

    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }

    fn get_label_data(&mut self, label: Label) -> &mut LabelData {
        match self.labels.get_mut(label) {
            Some(l) => l,
            None => panic!("label with id {} not found", label),
        }
    }

    pub fn create_label(&mut self) -> Label {
        let label = self.labels.len();
        self.labels.push(LabelData::new());
        label
    }

    pub fn label_here(&mut self, label: Label) {
        let target = self.ops.len();
        self.get_label_data(label).set_target(target);
    }

    pub fn push_jump(&mut self, label: Label, jump: Op) {
        let i = self.ops.len();
        self.get_label_data(label).jumps.push(i);
        match jump {
            Op::Jump(_) | Op::JumpZero(_) | Op::JumpNeg(_) => {}
            _ => panic!(
                "expected jump op, but found {} op",
                jump.get_type().get_name()
            ),
        }
        self.ops.push(jump);
    }

    pub fn into_vec(self) -> Vec<Op> {
        let mut ops = self.ops;
        for label in self.labels {
            let target = label.get_target() as i32;
            for jump in label.jumps {
                let target = target - jump as i32;
                match &mut ops[jump] {
                    Op::Jump(j) => j.dest = target,
                    Op::JumpZero(j) => j.dest = target,
                    Op::JumpNeg(j) => j.dest = target,
                    _ => unreachable!(),
                }
            }
        }
        ops
    }

    // loop methods

    pub fn loop_enter(&mut self, loop_id: Option<usize>) {
        let label_continue = self.create_label();
        let label_break = self.create_label();
        self.loop_stack.push((label_continue, label_break));
        if let Some(loop_id) = loop_id {
            match self.loops.insert(loop_id, (label_continue, label_break)) {
                Some(_) => panic!("loop with id {} already exists", loop_id),
                None => {}
            }
        }
    }

    pub fn loop_exit(&mut self, loop_id: Option<usize>) {
        self.loop_stack.pop();
        if let Some(loop_id) = loop_id {
            match self.loops.remove(&loop_id) {
                Some(_) => {}
                None => panic!("failed to exit loop with id {}", loop_id),
            }
        }
    }

    fn get_loop_labels(&self, loop_id: Option<usize>) -> (Label, Label) {
        if let Some(loop_id) = loop_id {
            match self.loops.get(&loop_id) {
                Some(l) => *l,
                None => panic!("cannot find loop with id {}", loop_id),
            }
        } else {
            match self.loop_stack.last() {
                Some(l) => *l,
                None => panic!("cannot get label: not in a loop block"),
            }
        }
    }

    pub fn loop_get_continue(&self, loop_id: Option<usize>) -> Label {
        self.get_loop_labels(loop_id).0
    }

    pub fn loop_get_break(&self, loop_id: Option<usize>) -> Label {
        self.get_loop_labels(loop_id).1
    }

    // var methods

    fn get_next_var_index(&mut self) -> u8 {
        match self.dropped.pop() {
            Some(i) => i,
            None => {
                if self.next_index == u8::MAX {
                    panic!("too many active variables for code generation");
                }
                let i = self.next_index;
                self.next_index += 1;
                i
            }
        }
    }

    pub fn bind_var(&mut self, var: Var) {
        let index = self.get_next_var_index();
        match self.vars.insert(var, index) {
            Some(_) => panic!("variable with id {} has already been bound", var),
            None => {}
        }
    }

    pub fn drop_var(&mut self, var: Var) {
        match self.vars.remove(&var) {
            Some(index) => {
                self.dropped.push(index);
            }
            None => panic!("failed to drop variable with id {}", var),
        }
    }

    fn get_var_index(&self, var: Var) -> u8 {
        match self.vars.get(&var) {
            Some(i) => *i,
            None => panic!("cannot find variable with id {}", var),
        }
    }

    pub fn push_var_load(&mut self, var: Var) {
        let index = self.get_var_index(var);
        self.push(ops::StackLoad::new(index).into());
    }

    pub fn push_var_store(&mut self, var: Var) {
        let index = self.get_var_index(var);
        self.push(ops::StackStore::new(index).into());
    }
}
