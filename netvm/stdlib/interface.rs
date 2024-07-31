

pub struct Incoming {
    pub to: String,
    pub data: Vec<u8>
}


// #[derive(Clone)]
pub struct Outgoing {
    pub is_success: bool,
    pub data: Vec<u8>,
}

impl Outgoing {
    pub fn default() -> Outgoing {
        Outgoing { is_success: false, data: Vec::new() }
    }
}
