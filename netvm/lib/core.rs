

pub enum Var {
    Number(f32),
    String(String)
}

impl Var {
    pub fn to_number(self) -> f32 {
        match self {
            Self::Number(v) => v,
            _ => panic!("Parsing number(f32) from var failed")
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Self::String(s) => s,
            _ => panic!("Parsing String from var failed")
        }
    }
}

#[derive(Clone)]
pub enum Opcode {
    Add,

    ToString,

    Println,

    Exit
}

#[derive(PartialEq)]
pub enum ReturnType { Exit }

pub struct Return {
    pub return_type: ReturnType,
    pub stack: Stack,
    pub prog: Vec<Opcode>
}

pub struct Stack (Vec<Var>);

impl Stack {
    pub fn new(start_stack: Vec<Var>) -> Stack {
        Stack(start_stack)
    }
}
impl Stack {
    pub fn take(&mut self) -> Var {
        self.0.pop().unwrap()
    }

    pub fn push(&mut self, var: Var) {
        self.0.push(var);
    }
}

impl Stack {
    pub fn run(mut self, prog: Vec<Opcode>) -> Return {
        for op in prog.clone() {
            match op {
                Opcode::Add => {
                    let x = self.take().to_number();
                    let y = self.take().to_number();
                    let result = x + y;
                    self.push(Var::Number(result));
                }

                Opcode::ToString => {
                    let var = self.take();
                    let string = match var {
                        Var::Number(num) => format!("{num}"),
                        Var::String(s) => s
                    };

                    self.push(Var::String(string));
                }

                Opcode::Println => {
                    let string = self.take().to_string();
                    println!("{string}");
                }


                Opcode::Exit => {
                    return Return { return_type: ReturnType::Exit, stack: self, prog }
                } }
        }

        panic!("Stack return fatal error!")
    }
}
