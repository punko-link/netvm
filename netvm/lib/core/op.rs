use lib::core::{ Stack, Return, ReturnType, Opcode, Var };
use lib::ops;

impl Stack {
    pub fn run(mut self, prog: Vec<Opcode>) -> Return {
        for op in prog.clone() {
            match op {
                Opcode::Add => {
                    let x = self.take(); let y = self.take();
                    self.push(ops::math::add(x, y));
                },

                Opcode::Dif => {
                    let x = self.take(); let y = self.take();
                    self.push(ops::math::dif(x, y));
                }

                Opcode::Mul => {
                    let x = self.take(); let y = self.take();
                    self.push(ops::math::mul(x, y));
                }

                Opcode::Div => {
                    let x = self.take(); let y = self.take();
                    self.push(ops::math::div(x, y));
                }

                Opcode::ToString => {
                    let v = self.take();
                    self.push(Var::String(ops::convert::to_string(v)))
                },

                Opcode::ToNumber => {
                    let v = self.take();
                    self.push(Var::Number(ops::convert::to_number(v)))
                }

                Opcode::ToUint => {
                    let v = self.take();
                    self.push(Var::Uint(ops::convert::to_uint(v)))
                }


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
