use lib::core::{ Stack, Return, ReturnType, Opcode, Var };
use lib::ops;


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
                    let x = self.take(); let y = self.take();
                    self.push(ops::math::add(x, y));
                },

                Opcode::ToString =>  self.push(Var::String(ops::convert::to_string(self.take()))),

                Opcode::Println => {
                    let string = ops::convert::to_string(self.take());
                    println!("{string}");
                }


                Opcode::Exit => {
                    return Return { return_type: ReturnType::Exit, stack: self, prog }
                } }
        }

        panic!("Stack return fatal error!")
    }
}
