use lib::core::Var;
use lib::ops;

pub fn add(x: Var, y: Var) -> Var {
    match x {
        Var::Number(n) => {
            let r = n + ops::convert::to_number(y);
            Var::Number(r)
        }
        Var::Uint(n) => {
            let r = n + ops::convert::to_uint(y);
            Var::Uint(r)
        }

        _ => panic!("Add operation is allowed for Number and Uint only!")
    }
}
