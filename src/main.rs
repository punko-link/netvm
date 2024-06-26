use data::{Instruction, Stack, Var};
use data::buffer::CMD__ADD;

fn main() {
    let mut stack = Stack::new();
    stack = stack.push(Var::from_f32(10.0));
    stack = stack.push(Var::from_f32(10.0));
    stack = stack.push(Var::Cmd(CMD__ADD));
    let mut instr: Option<Instruction> = None;
    (stack, instr) = stack.execute();

    let result = stack.take();
    let num = result.to_f32();
    println!("{num}");
}
