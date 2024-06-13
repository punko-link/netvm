use crate::{Graph, List, Var};
use crate::buffer::{BUFFER_LENGTH_NUMBER, BUFFER_LENGTH_POINTER, BUFFER_LENGTH_UINT, DATATYPE_CODE_BOOL, DATATYPE_CODE_BYTE, DATATYPE_CODE_CMD, DATATYPE_CODE_NUMBER, DATATYPE_CODE_POINTER, DATATYPE_CODE_STRING, DATATYPE_CODE_UINT, DATATYPE_CODE_UNDEFINED};

impl Var {
    pub fn from_buffer(data_type_code: u8, mut buffer: Vec<u8>) -> Var {
        match data_type_code {
            DATATYPE_CODE_UNDEFINED => Var::Undefined,
            DATATYPE_CODE_NUMBER => {
                let slice: [u8; 4] = TryFrom::try_from(&*buffer).unwrap();
                Var::Number(f32::from_be_bytes(slice))
            }
            DATATYPE_CODE_UINT => {
                let slice: [u8; 8] = TryFrom::try_from(&*buffer).unwrap();
                Var::Uint(u64::from_be_bytes(slice) as usize)
            }
            DATATYPE_CODE_POINTER => {
                let slice: [u8; 8] = TryFrom::try_from(&*buffer).unwrap();
                Var::Pointer(u64::from_be_bytes(slice) as usize)
            }
            DATATYPE_CODE_STRING => {
                Var::String(String::from_utf8(buffer).unwrap())
            }
            DATATYPE_CODE_BYTE => {
                Var::Byte(buffer[0])
            },
            DATATYPE_CODE_BOOL => {
                Var::Bool(buffer[0] != 0)
            },
            DATATYPE_CODE_CMD => {
                Var::Cmd(buffer[0])
            },

            _ => panic!("Type parsing Error")
        }
    }
}

impl List {
    pub fn from_buffer(mut buffer: Vec<u8>) -> List {
        let mut list: List = List::new();

        let mut key: String = String::new();
        let mut data_type: u8 = 0b1111_1111;
        let mut len: usize = 0;
        let mut len_vec: Vec<u8> = Vec::new();
        let mut data_buffer: Vec<u8> = Vec::new();

        for byte in buffer {
            if !key.is_empty() {

            }
            else if data_type == 0b1111_1111 {
                data_type = byte;
                match data_type {
                    DATATYPE_CODE_NUMBER => len = BUFFER_LENGTH_NUMBER,
                    DATATYPE_CODE_UINT => len = BUFFER_LENGTH_UINT,
                    DATATYPE_CODE_POINTER => len = BUFFER_LENGTH_POINTER,
                    _ => {}
                }
            }
            else if data_type != 0b1111_1111 && len == 0 {
                len_vec.push(byte);
                if len_vec.len() == 8 {
                    let slice: [u8; 8] = TryFrom::try_from(&*len_vec).unwrap();
                    len = u64::from_be_bytes(slice) as usize
                }
            }
            else {
                data_buffer.push(byte);
                if data_buffer.len() == len {
                    list = list.set(key.clone(), Var::from_buffer(data_type, data_buffer.clone()));
                    key = String::new();
                    data_type = 0b1111_1111;
                    len = 0;
                    len_vec = Vec::new();
                    data_buffer = Vec::new();
                }
            }
        }

        list
    }
}

impl Graph {
    pub fn from_buffer(buffer: Vec<u8>) -> Graph {
        let mut graph = Graph::new();

        let mut len: usize = 0;
        let mut len_vec: Vec<u8> = Vec::new();
        let mut data_vector: Vec<u8> = Vec::new();

        for byte in buffer {
            if len == 0 {
                len_vec.push(byte);
                if len_vec.len() == 8 {
                    let slice: [u8; 8] = TryFrom::try_from(&*len_vec).unwrap();
                    len = u64::from_be_bytes(slice) as usize
                }
            }
            else {
                data_vector.push(byte);
                if data_vector.len() == len {
                    let list = List::from_buffer(data_vector.clone());
                    let (g, _) = graph.clone().create_node(list);
                    graph = g;
                }
            }
        }

        graph
    }
}
