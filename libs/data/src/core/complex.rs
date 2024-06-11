use crate::{ Var };


impl Var {
    pub fn to_vec(self) -> Vec<Var> {
        match self {
            Self::Vector(s) => s,
            _ => panic!("Type Error")
        }
    }

    pub fn from_vec(v: Vec<Var>) -> Var {
        Var::Vector(v)
    }
}
