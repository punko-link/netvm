use crate::Var;

impl Var {
    pub fn to_f32(self) -> f32 {
        match self {
            Self::Number(s) => s,
            _ => panic!("Type Error")
        }
    }

    pub fn from_f32(v: f32) -> Var {
        Var::Number(v)
    }
}

impl Var {
    pub fn to_uint(self) -> usize {
        match self {
            Self::Uint(s) => s,
            _ => panic!("Type Error")
        }
    }

    pub fn from_uint(v: usize) -> Var {
        Var::Uint(v)
    }
}



