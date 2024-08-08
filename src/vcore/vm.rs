use crate::dist::{PROG_LEN, STACK_SIZE};
use crate::sys_gnu_x86_64::process::exit::{control_panic};
use crate::vcore::ops::Opcode;
use crate::vcore::point::Point;

pub struct VM {
    prog: [Opcode; PROG_LEN],
    op_select: usize,
    stack: [Point; STACK_SIZE]
}

impl VM {
    pub fn new(prog: [Opcode; PROG_LEN], stack: [Point; STACK_SIZE]) -> VM { VM { prog, op_select: 0, stack }}
}

impl VM {
    pub fn select_op(mut self) -> Opcode {
        if self.op_select >= PROG_LEN {  control_panic("op_select out of prog.lan") }
        let op = self.prog.get(self.op_select).unwrap();
        self.op_select += 1;

        *op
    }

    // fn stack_tack(mut self, start: usize, count: usize) -> (Point, Point) {
    //     for (i, point) in self.stack.iter().enumerate() {
    //
    //     };
    // }

    pub fn tic(mut self) {
        match self.select_op() {
            Opcode::Add => {
                // let ax = self
            }
            Opcode::Log => {}
            Opcode::Exit => {}
            Opcode::Panic => {}
        }

    }
}
