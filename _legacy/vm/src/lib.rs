pub mod bytecode;
pub mod datamodel;

mod callframe;
mod callstack;
mod vm;

use callframe::CallFrame;
use callstack::CallStack;

pub use vm::{VirtualMachine, VmState};
