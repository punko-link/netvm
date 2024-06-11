use crate::Var;

impl Var {
    pub fn value_of_pointer(self) -> usize {
        match self {
            Self::Pointer(addr) => addr,
            _ => panic!("Type Error")
        }
    }

    pub fn pointer(addr: usize) -> Var {
        Var::Pointer(addr)
    }
}
