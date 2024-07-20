use core::{ Opcode, Var };

pub fn prog() -> Vec<Opcode> {
    vec![
        Opcode::Div,
        Opcode::ToString,
        Opcode::Println,
        Opcode::Exit
    ]
}

pub fn stack() -> Vec<Var> {
    vec![
        Var::Number(10.0),
        Var::Number(5.0)
    ]

}
