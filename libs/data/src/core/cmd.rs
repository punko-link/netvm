use crate::Var;

impl Var {
    pub fn is_cmd(self) -> bool {
        match self {
            Self::Cmd(..) => true,
            _ => false
        }
    }

    pub fn cmd_to_byte(self) -> u8 {
        match self {
            Self::Cmd(byte) => byte,
            _ => panic!("Type mismatch")
        }
    }
}
