use crate::var::{Var, VarType};

impl Var {
    pub fn type_of(self) -> String {
        String::from(match self.var_type {
            VarType::Buffer => "buffer",
            VarType::Integer => "integer",
            VarType::Float => "float",
            VarType::Byte => "byte",
            VarType::True => "bool",
            VarType::False => "bool",
            VarType::Object => "object",
            VarType::Undefined => "undefined"
        })
    }
}