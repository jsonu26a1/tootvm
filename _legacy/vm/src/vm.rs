use std::mem;

use crate::bytecode::{OpAction, OpError};
use crate::datamodel::{Function, Value};

use super::CallFrame;

pub struct VirtualMachine {
    frame: Option<Box<CallFrame>>,
}

impl VirtualMachine {
    pub fn new(func: Function) -> VirtualMachine {
        VirtualMachine {
            frame: Some(Box::new(CallFrame::new(func))),
        }
    }

    pub fn run_until_exited(&mut self) -> Result<Value, OpError> {
        loop {
            let action = self.step()?;
            match self.process(action)? {
                VmState::Running => continue,
                VmState::Exited(val) => return Ok(val),
            }
        }
    }

    pub fn step(&mut self) -> Result<OpAction, OpError> {
        let frame = self.frame.as_mut().unwrap();
        frame.exec()
    }

    pub fn process(&mut self, action: OpAction) -> Result<VmState, OpError> {
        match action {
            OpAction::None => (),
            OpAction::Jump(dest) => {
                let frame = self.frame.as_mut().unwrap();
                frame.jump(dest);
            }
            OpAction::Call(func, args) => {
                let mut callee = Box::new(CallFrame::new(func));
                // NOTE: for expr `Call(A, B, C)`, args is reversed: `[C, B, A]`
                // so now the order that they will be popped off the stack is
                // (A, B, C), which is how the stage0 compiler expects them.
                // see crate::bytecode::ops::Call for details
                for arg in args.into_iter() {
                    callee.push(arg);
                }
                mem::swap(&mut self.frame, &mut callee.parent);
                self.frame = Some(callee);
            }
            OpAction::CallNative(func, args) => {
                let frame = self.frame.as_mut().unwrap();
                frame.push(func(args));
            }
            OpAction::Return(val) => {
                let frame = self.frame.as_mut().unwrap();
                let mut parent = None;
                mem::swap(&mut frame.parent, &mut parent);
                match parent {
                    Some(mut parent) => {
                        parent.push(val);
                        self.frame = Some(parent);
                    }
                    None => {
                        self.frame = None;
                        return Ok(VmState::Exited(val));
                    }
                }
            }
        }
        Ok(VmState::Running)
    }
}

pub enum VmState {
    Running,
    Exited(Value),
}
