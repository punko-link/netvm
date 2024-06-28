use data::{Instruction, Stack, Var};
use data::buffer::CMD__ADD;

fn main() {
    let mut stack = Stack::new();
    stack = stack.push(Var::pointer(10));
    stack = stack.push(Var::pointer(10));
    stack = stack.push(Var::Cmd(CMD__ADD));
    let mut instr: Option<Instruction> = None;
    (stack, instr) = stack.execute();

    let result = stack.take();
    let num = result.value_of_pointer();
    println!("{num}");
}
