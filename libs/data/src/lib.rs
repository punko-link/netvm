mod core;
pub mod buffer;
mod ops_stack;

use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Var {
    Byte(u8),
    Bool(bool),
    Number(f32),
    Uint(usize),
    Pointer(usize),
    String(String),
    Undefined,
    Cmd(u8),

}


#[derive(Clone, PartialEq)]
pub struct List (HashMap<String, Var>);

#[derive(Clone, PartialEq)]
pub struct Graph (Vec<List>);


pub struct Instruction {
    args: Vec<Var>,
    cmd: u8
}

#[derive(Clone)]
pub struct Stack (Vec<Var>);


