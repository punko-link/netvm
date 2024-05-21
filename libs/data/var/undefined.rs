use crate::var::{Var, VarType};


impl Var {
    pub fn return_undefined(self) -> Var {
        Var { single_data: Box::new(None), vector_data: Box::new(None), var_type: VarType::Undefined }
    }
}