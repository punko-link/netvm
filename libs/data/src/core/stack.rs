use crate::{Instruction, Stack, Var};
use crate::ops_stack::add::add;
use crate::buffer::{CMD__ADD};
impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::new())
    }
}

impl Stack {
    pub fn push(mut self, var: Var) -> Stack {
        self.0.push(var);
        self
    }

    pub fn take(mut self) -> Var {
        let addr = self.0.len() - 1;
        self.0.remove(addr)
    }
}


impl Stack {
    pub fn take_until_cmd(mut self) -> (Stack, Instruction) {
        let mut args: Vec<Var> = Vec::new();

        for one in self.clone().0 {
            if one.clone().is_cmd() {
                return (self, Instruction { args, cmd: one.cmd_to_byte() })
            }

            else {
                args.push(one);
            }
        }
        // loop {
        //     let addr = self.0.len() - 1;
        //     println!("111 {}", addr);
        //     let one = self.0.remove(addr);
        //     if one.clone().is_cmd() {
        //         return (self, Instruction { args, cmd: one.cmd_to_byte() })
        //     }
        //
        //     else {
        //         args.push(one);
        //     }
        //     println!("444");
        //     if addr == 0 { break };
        // }

        panic!("NO CMD IN STACK");
    }
}

impl Stack {
    pub fn execute(self) -> (Stack, Option<Instruction>) {
        let mut stack = self;

        let instr: Instruction;
        (stack, instr) = stack.take_until_cmd();

        let result =  match instr.cmd {
            CMD__ADD => add(instr.args),
            _ => return (stack, Some(instr))
        };

        stack = stack.push(result);
        (stack, None)
    }
}
