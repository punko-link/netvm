use core::{ ops, Var};

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


pub fn dif(x: Var, y: Var) -> Var {
    match x {
        Var::Number(n) => {
            let r = n - ops::convert::to_number(y);
            Var::Number(r)
        }
        Var::Uint(n) => {
            let r = n - ops::convert::to_uint(y);
            Var::Uint(r)
        }

        _ => panic!("Add operation is allowed for Number and Uint only!")
    }
}

pub fn mul(x: Var, y: Var) -> Var {
    match x {
        Var::Number(n) => {
            let r = n * ops::convert::to_number(y);
            Var::Number(r)
        }
        Var::Uint(n) => {
            let r = n * ops::convert::to_uint(y);
            Var::Uint(r)
        }

        _ => panic!("Add operation is allowed for Number and Uint only!")
    }
}

pub fn div(x: Var, y: Var) -> Var {
    match x {
        Var::Number(n) => {
            let r = n / ops::convert::to_number(y);
            Var::Number(r)
        }
        Var::Uint(n) => {
            let r = n / ops::convert::to_uint(y);
            Var::Uint(r)
        }

        _ => panic!("Add operation is allowed for Number and Uint only!")
    }
}

