
use std::collections::HashMap;
use data::buffer::{CMD__ADD, CMD_CONSOLE_LOG, DATATYPE_CODE_NUMBER};

pub struct Dist {
    instructions: HashMap<String, Vec<u8>>
}

impl Dist {
    pub fn init() -> Dist { 
        let mut dist = Dist { instructions: HashMap::new() };

        dist.instructions.insert("sum".to_string(), Vec::from([
            CMD__ADD,
            DATATYPE_CODE_NUMBER,
            0b01000001, 0b00100000, 0b00000000, 0b00000000, //10 - arg1
            DATATYPE_CODE_NUMBER,
            0b01000001, 0b00100000, 0b00000000, 0b00000000, //10 - arg2
            DATATYPE_CODE_NUMBER,
            0b00000000, 0b00000000, 0b00000000, 0b00000000, // 0 - 0 to write result
            CMD_CONSOLE_LOG


        ]));
        dist
    }
}
