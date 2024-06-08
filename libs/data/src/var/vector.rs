use crate::var::Var;

impl Var {
    pub fn buffer_to_vec_u8(self) -> Vec<u8> {
        match self {
            Self::Buffer(s) => s,
            _ => panic!("buffer_to_vec_u8 allowed for Var::Buffer only!")
        }
    }


    pub fn vector_to_var_vec(self) -> Vec<Var> {
        match self {
            Self::Vector(s) => s,
            _ => panic!("vector_to_var_vec allowed for Var::Vector only!")
        }
    }
}
