use crate::var::Var;

impl Var {
    pub fn undefined() -> Var { Var::Undefined }

    pub fn is(self) -> bool {
        match self {
            Self::Undefined => false,
            _ => true
        }
    }
}
