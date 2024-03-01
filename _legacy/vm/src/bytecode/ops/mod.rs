#[macro_use]
mod macros;

mod buffer;
mod call;
mod cmp;
mod int;
mod jump;
mod list;
mod literal;
mod num;
mod real;
mod seq;
mod stack;
mod table;
mod tuple;

use super::{BytesIO, BytesReadError, DataIO, OpAction, OpError, Operation};

use crate::CallStack;

pub use buffer::{BufferCreate, BufferGetSlice, BufferSetSlice};
pub use call::{Call, Return};
pub use cmp::{Cmp, GetType};
pub use int::{And, Not, Or, Shl, Shr, Xor};
pub use jump::{Jump, JumpNeg, JumpZero};
pub use list::{ListCreate, ListGetSlice, ListPop, ListPush};
pub use literal::{LiteralCreate, LiteralValue};
pub use num::{Add, Div, Mul, Neg, Rem, Sub};
pub use real::{Ceil, Floor, IntToReal, Round, Trunc};
pub use seq::{SeqAppend, SeqGet, SeqLen, SeqResize, SeqSet, SeqToList};
pub use stack::{StackCopy, StackLoad, StackPop, StackStore, StackSwap};
pub use table::TableCreate;
pub use tuple::{TupleCreate, TupleFromList, TupleWeakRef, TupleWeakUpgrade};
