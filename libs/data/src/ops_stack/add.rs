use crate::Var;

pub fn add(args: Vec<Var>) -> Var {
    let x = args.get(0).expect("Stack Error").clone();
    let y = args.get(1).expect("Stack Error").clone();
    let data_type = x.clone().check_type_match(y.clone());

    match x {
        Var::Number(x) => {
            let result: f32 = x + y.to_f32();
            Var::from_f32(result)
        },
        Var::Uint(x) => {
            let result: usize = x + y.to_uint();
            Var::from_uint(result)
        },
        Var::Pointer(x) => {
            let result: usize = x + y.value_of_pointer();
            Var::pointer(result)
        }

        _ => panic!("{}", format!("Add is not allowed for {data_type}"))
    }
}
