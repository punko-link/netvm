use std::ops::{Add, Div, Mul, Sub};
use crate::var::Var;

impl Add<f64> for Var {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        let n = match self {
            Self::Number(s) => Some(s),
            _ => panic!("add allowed for numbers only")
        };

        n.unwrap() + rhs
    }
}

impl Sub<f64> for Var {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        let n = match self {
            Self::Number(s) => Some(s),
            _ => panic!("sub allowed for numbers only")
        };

        n.unwrap() - rhs
    }
}


impl Mul<f64> for Var {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        let n = match self {
            Self::Number(s) => Some(s),
            _ => panic!("mul allowed for numbers only")
        };

        n.unwrap() * rhs
    }
}


impl Div<f64> for Var {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        let n = match self {
            Self::Number(s) => Some(s),
            _ => panic!("div allowed for numbers only")
        };

        n.unwrap() / rhs
    }
}
