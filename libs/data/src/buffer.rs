mod serialize;
mod deserialize;


pub const DATATYPE_CODE_UNDEFINED: u8 = 0b0000_0000;
pub const DATATYPE_CODE_NUMBER: u8 = 0b0000_0001;
pub const DATATYPE_CODE_UINT: u8 = 0b0000_0010;
pub const DATATYPE_CODE_POINTER: u8 = 0b0000_0011;
pub const DATATYPE_CODE_LIST: u8 = 0b0000_0100;
pub const DATATYPE_CODE_GRAPH: u8 = 0b0000_0101;
pub const DATATYPE_CODE_STRING: u8 = 0b0000_0110;

pub const BUFFER_LENGTH_NUMBER: usize = 4;
pub const BUFFER_LENGTH_UINT: usize = 8;
pub const BUFFER_LENGTH_POINTER: usize = 8;


