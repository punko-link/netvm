pub mod math;
pub mod convert;


use core::{ Stack, Return, ReturnType, Opcode, Var };

impl Stack {
    pub fn run(mut self, prog: Vec<Opcode>) -> Return {
        for op in prog.clone() {
            match op {
                Opcode::Add => {
                    let x = self.take(); let y = self.take();
                    self.push(math::add(x, y));
                },

                Opcode::Dif => {
                    let x = self.take(); let y = self.take();
                    self.push(math::dif(x, y));
                }

                Opcode::Mul => {
                    let x = self.take(); let y = self.take();
                    self.push(math::mul(x, y));
                }

                Opcode::Div => {
                    let x = self.take(); let y = self.take();
                    self.push(math::div(x, y));
                }

                Opcode::ToString => {
                    let v = self.take();
                    self.push(Var::String(convert::to_string(v)))
                },

                Opcode::ToNumber => {
                    let v = self.take();
                    self.push(Var::Number(convert::to_number(v)))
                }

                Opcode::ToUint => {
                    let v = self.take();
                    self.push(Var::Uint(convert::to_uint(v)))
                }


                Opcode::Println => {
                    let string = convert::to_string(self.take());
                    println!("{string}");
                }

                Opcode::Exit => {
                    return Return { return_type: ReturnType::Exit, stack: self, prog }
                } }
        }

        panic!("Stack return fatal error!")
    }
}
