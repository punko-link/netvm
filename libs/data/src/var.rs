mod number;
mod string;
mod vector;
mod pointer;
mod undefined;

use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Var {
    Pointer(usize),
    Number(f64), // +
    Buffer(Vec<u8>), // +
    String(String), // +
    Vector(Vec<Var>), // +
    Graph(Graph),
    Object(Object),
    Undefined
}

#[derive(Clone, PartialEq)]
pub struct Graph {}


#[derive(Clone, PartialEq)]
pub struct Object {
    data: HashMap<String, Var>
}


// impl Object {
//     pub fn new() -> Object { Object { data: HashMap::new() } }
//
//     pub fn set(mut self, key: String, value: Var) {
//         self.data.insert(key, value);
//     }
//
//     pub fn get(self, name: String) -> Var {
//         let data = self.data.get(&name);
//         if data.is_none() { return Var::Undefined };
//
//         data.unwrap().clone()
//     }
//
//     pub fn rm(mut self, name: String) -> Var {
//         let data = self.data.remove(&name);
//         if data.is_none() { return  Var::Undefined };
//
//         data.unwrap()
//     }
// }
//
// impl Object {

// }



pub fn a() {
    // let mut c = Var::String("Hallo".to_string());
    // let d = Var::String("World".to_string());
    // c.append(d);
    // println!("{c}");
}
