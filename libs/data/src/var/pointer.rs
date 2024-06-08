use crate::var::Var;

impl Var {
    pub fn address(self) -> usize {
        match self {
            Self::Pointer(s) => s,
            _ => panic!("pointer -> usize address allowed for Var::Pointer only!")
        }
    }
}
