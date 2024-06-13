use crate::Var;

impl Var {
    pub fn type_name(self) -> *const str {
        match self {
            Self::Pointer(..) => "pointer",
            Self::Number(..) => "number",
            Self::Uint(..) => "uint",
            Self::String(..) => "string",
            Self::Byte(..) => "byte",
            Self::Bool(..) => "bool",
            Self::Cmd(..) => "cmd",
            Self::Undefined => "undefined"
        }
    }
}
