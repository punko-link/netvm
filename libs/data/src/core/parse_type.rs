use crate::Var;

impl Var {
    pub fn type_name(self) -> *const str {
        match self {
            Self::Pointer(..) => "pointer",
            // Self::Vector(..) => "vector",
            Self::Number(..) => "number",
            Self::Uint(..) => "uint",
            Self::String(..) => "string",
            Self::Undefined => "undefined"
        }
    }
}
