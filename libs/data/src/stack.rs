use crate::{Instruction, Stack, Var};

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
    pub fn take_until_cmd(mut self) -> Instruction {
        let mut args: Vec<Var> = Vec::new();
        loop {
            let addr = self.0.len() - 1;
            if addr == -1 { break };
            match self.0.remove(addr) {
                Var::Cmd(cmd) => {
                    return Instruction { args, cmd }
                },
                (arg) => args.push(arg.clone())
            }
        }

        panic!("NO CMD IN STACK");
    }
}
