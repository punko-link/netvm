use crate::Var;

impl Var {
    pub fn to_string(self) -> String {
        match self {
            Self::String(s) => s,
            _ => panic!("Type Error")
        }
    }

    pub fn from_string(v: String) -> Var {
        Var::String(v)
    }

}
