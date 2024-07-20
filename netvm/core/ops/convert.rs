use core::Var;

pub fn to_number(v: Var) -> f32 {
    match v {
        Var::Number(v) => v,
        Var::Uint(v) => v as f32,
        Var::String(v) => v.parse::<f32>().expect("Failed to parse Number(f32) from Var"),
    }
}

pub fn to_uint(v: Var) -> u64 {
    match v {
        Var::Uint(v) => v,
        Var::Number(v) => v as u64,
        Var::String(v) => v.parse::<u64>().expect("Failed to parse Uint(u64) from Var"),
    }
}

pub fn to_string(v: Var) -> String {
    match v {
        Var::String(s) => s,
        Var::Number(num) => format!("{num}"),
        Var::Uint(num) => format!("{num}"),
    }
}
