use data::var::Var;

fn data_point_example() {
    let vec_u8_point = Var::buffer(Vec::from("Hallo"));
    let vec_u8 = vec_u8_point.value_of_buffer();
    println!("{}", String::from_utf8(vec_u8).unwrap());


    let integer_point = Var::integer(1984);
    let integer = integer_point.value_of_integer();
    println!("{integer}");

    let float_point = Var::float(1984.2);
    let float = float_point.value_of_float();
    println!("{float}");


    let byte_point = Var::byte(255);
    let byte = byte_point.value_of_byte();
    println!("{byte}");

    let flag_point = Var::boolean(true);
    let boolean = flag_point.value_of_boolean();
    println!("{boolean}");
}


fn main() {
    data_point_example();
}