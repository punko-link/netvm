use crate::var::{Var, VarType};

impl Var {
    pub fn value_of_buffer(self) -> Vec<u8> {
        self.single_data.unwrap()
    }

    pub fn value_of_integer(self) -> i64 {
        let slice: [u8; 8] = <[u8; 8]>::try_from(self.single_data.unwrap().as_slice()).unwrap();
        i64::from_be_bytes(slice)
    }

    pub fn value_of_float(self) -> f64 {
        let slice: [u8; 8] = <[u8; 8]>::try_from(self.single_data.unwrap().as_slice()).unwrap();
        f64::from_be_bytes(slice)
    }

    pub fn value_of_byte(self) -> u8 {
        self.single_data.unwrap().get(0).unwrap().clone()
    }

    pub fn value_of_boolean(self) -> bool {
        match self.var_type {
            VarType::False => false,
            _ => true
        }
    }
}