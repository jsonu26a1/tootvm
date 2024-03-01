extern crate peanut_script_vm as vm;

pub mod stage0;
pub mod stage1;

/*
The compiler has many stages, which execute in descending order.

    Stage 0 focuses entirely on bytecode generation. It doesn't check the
types of expressions, and provides only a few basic language abstractions
on top of the bytecode operations, like loops and if-else statements.

    Stage 1 is currently in development. It will perform type checking, and
will convert the syntax tree for input into the stage 0 compiler.
*/
