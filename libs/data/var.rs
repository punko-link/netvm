use std::collections::HashMap;

mod create;
mod value_of;
mod type_of;
mod object;
mod undefined;


#[derive(Clone)]
pub struct Object {
    data: HashMap<String, Var>
}


#[derive(Clone)]
#[derive(PartialEq)]
pub enum VarType { Buffer, Integer, Float, Byte, True, False, Object, Undefined }

#[derive(Clone)]
pub struct Var {
    single_data: Box<Option<Vec<u8>>>,
    vector_data: Box<Option<Object>>,
    var_type: VarType
}
