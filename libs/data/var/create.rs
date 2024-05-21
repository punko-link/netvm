use crate::var::{Var, VarType};

impl Var {
    pub fn buffer(vector: Vec<u8>) -> Var {
        Var { single_data: Box::from(Some(vector)), vector_data: Box::new(None), var_type: VarType::Buffer }
    }

    pub fn integer(int: i64) -> Var {
        let vector = Vec::from(int.to_be_bytes().as_slice());
        Var { single_data: Box::from(Some(vector)), vector_data: Box::new(None), var_type: VarType::Integer }
    }

    pub fn float(float: f64) -> Var {
        let vector = Vec::from(float.to_be_bytes().as_slice());
        Var { single_data: Box::from(Some(vector)), vector_data: Box::new(None), var_type: VarType::Float }
    }

    pub fn byte(byte: u8) -> Var {
        let vector = Vec::from([byte]);
        Var { single_data: Box::from(Some(vector)), vector_data: Box::new(None), var_type: VarType::Byte }
    }

    pub fn boolean(flag: bool) -> Var {
        let flag = match flag {
            false => VarType::False,
            _ => VarType::True
        };
        Var { single_data: Box::from(None), vector_data: Box::new(None), var_type: flag }
    }
}