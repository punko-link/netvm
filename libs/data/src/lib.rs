mod core;
mod buffer;

use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Var {
    Number(f32),
    Uint(usize),
    Pointer(usize),
    Vector(Vec<Var>),
    String(String),
    Undefined
}


#[derive(Clone, PartialEq)]
pub struct List (HashMap<String, Var>);

#[derive(Clone, PartialEq)]
pub struct Graph (Vec<List>);

