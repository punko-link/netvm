mod core;
mod dist;

fn main() {
    let mut stack = core::Stack::new(dist::stack());
    let mut prog = dist::prog();

    loop {
        let result = stack.run(prog);
        if result.return_type == core::ReturnType::Exit {
            break;
        }

        stack = result.stack;
        prog = result.prog;
    }
}
