mod stack;


#[derive(Clone)]
pub enum Opcode {
    Add,

    ToString,

    Println,

    Exit
}

pub struct Stack (Vec<Var>);


#[derive(PartialEq)]
pub enum ReturnType { Exit }

pub struct Return {
    pub return_type: ReturnType,
    pub stack: Stack,
    pub prog: Vec<Opcode>
}

pub enum Var {
    Number(f32),
    Uint(u64),
    String(String)
}
