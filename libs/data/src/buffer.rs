mod serialize;
mod deserialize;


pub const DATATYPE_CODE_UNDEFINED: u8 = 0b0000_0000;

pub const DATATYPE_CODE_NUMBER: u8 = 0b0000_0001;
pub const DATATYPE_CODE_UINT: u8 = 0b0000_0010;
pub const DATATYPE_CODE_POINTER: u8 = 0b0000_0011;
pub const DATATYPE_CODE_LIST: u8 = 0b0000_0100;
pub const DATATYPE_CODE_GRAPH: u8 = 0b0000_0101;
pub const DATATYPE_CODE_STRING: u8 = 0b0000_0110;
pub const DATATYPE_CODE_BYTE: u8 = 0b0000_0111;
pub const DATATYPE_CODE_BOOL: u8 = 0b0000_1000;
pub const DATATYPE_CODE_CMD: u8 = 0b0000_1001;


pub const BUFFER_LENGTH_NUMBER: usize = 4;
pub const BUFFER_LENGTH_UINT: usize = 8;
pub const BUFFER_LENGTH_POINTER: usize = 8;


pub const CMD_UNCHECKED: u8 = 0b1000_0000;
pub const CMD__ADD: u8 = 0b1000_0001; // +
pub const CMD__DIF: u8 = 0b1000_0010; // -
pub const CMD__MUL: u8 = 0b1000_0011; // *
pub const CMD__PRV: u8 = 0b1000_0100; // /
pub const CMD__POW: u8  = 0b1000_0101; // ^
pub const CMD_ROOT: u8  = 0b1000_0110; // sqrt
pub const CMD_EQUL: u8 = 0b1000_0111; // =
pub const CMD_IS_EQUL: u8 = 0b1000_1000; // ==
pub const CMD_IS_BIG: u8 = 0b1000_1001; // >
pub const CMD_IS_BIG_OR_EQUL: u8 = 0b1000_1010; // >=
pub const CMD_IS_SML: u8 = 0b1000_1011; // <
pub const CMD_IS_SML_OR_EQUL: u8 = 0b1000_1100; // <=


pub const CMD_CONSOLE_LOG: u8 = 0b1000_1101; // println
