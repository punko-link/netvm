use crate::var::Var;

impl Var {
    pub fn to_string(self) -> String {
        match self {
            Self::String(s) => s,
            _ => panic!("to_string allowed for Var::String only!")
        }
    }
}
