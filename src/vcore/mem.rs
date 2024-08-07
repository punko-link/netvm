use crate::dist::{MEMORY_SIZE, START_STACK, START_STACK_LEN};

pub struct Memory {
    memory: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn init() -> Memory {
        let mut mem = Memory { memory: [0x0; MEMORY_SIZE] };
        for i in 0..START_STACK_LEN {
            mem.memory[i] = START_STACK[i];
        }

        mem
    }
}
