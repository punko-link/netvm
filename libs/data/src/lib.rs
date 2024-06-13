mod core;
mod buffer;

use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Var {
    Number(f32),
    Uint(usize),
    Pointer(usize),
    String(String),
    Undefined
}


#[derive(Clone, PartialEq)]
pub struct List (HashMap<String, Var>);

#[derive(Clone, PartialEq)]
pub struct Graph (Vec<List>);


enum PackEntry { Var(Var), List(List), Graph(Graph) }
pub struct Pack (Vec<PackEntry>);
