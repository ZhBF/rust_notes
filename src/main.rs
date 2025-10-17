
// The entry point of the program
fn main() {
    learn_print_lines();
    learn_data_types();
}


fn learn_print_lines() {
    println!("Hello, world!");
    println!("Hello, {}!", "Rust");
}

fn learn_data_types() {
    learn_scalar_types();
    learn_compound_types();
}

fn learn_scalar_types() {
    learn_integer_types();
    learn_floating_types();
    learn_boolean_types();
    learn_character_types();
}

fn learn_compound_types() {
    learn_tuple_types();
    learn_array_types();
}

fn learn_integer_types() {
    
    let i8_min: i8 = i8::MIN;
    let i8_max: i8 = i8::MAX;
    println!("i8 range: {} to {}", i8_min, i8_max);

    let u8_min: u8 = u8::MIN;
    let u8_max: u8 = u8::MAX;
    println!("u8 range: {} to {}", u8_min, u8_max);

    let i16_min: i16 = i16::MIN;
    let i16_max: i16 = i16::MAX;
    println!("i16 range: {} to {}", i16_min, i16_max);

    let u16_min: u16 = u16::MIN;
    let u16_max: u16 = u16::MAX;
    println!("u16 range: {} to {}", u16_min, u16_max);

    let i32_min: i32 = i32::MIN;
    let i32_max: i32 = i32::MAX;
    println!("i32 range: {} to {}", i32_min, i32_max);

    let u32_min: u32 = u32::MIN;
    let u32_max: u32 = u32::MAX;
    println!("u32 range: {} to {}", u32_min, u32_max);

    let i64_min: i64 = i64::MIN;
    let i64_max: i64 = i64::MAX;
    println!("i64 range: {} to {}", i64_min, i64_max);

    let u64_min: u64 = u64::MIN;
    let u64_max: u64 = u64::MAX;
    println!("u64 range: {} to {}", u64_min, u64_max);

    let i128_min: i128 = i128::MIN;
    let i128_max: i128 = i128::MAX;
    println!("i128 range: {} to {}", i128_min, i128_max);

    let u128_min: u128 = u128::MIN;
    let u128_max: u128 = u128::MAX;
    println!("u128 range: {} to {}", u128_min, u128_max);

    let isize_min: isize = isize::MIN;
    let isize_max: isize = isize::MAX;
    println!("isize range: {} to {}", isize_min, isize_max);

    let usize_min: usize = usize::MIN;
    let usize_max: usize = usize::MAX;
    println!("usize range: {} to {}", usize_min, usize_max);

    let i_decimal = 98_222;
    let i_hex = 0xff;
    let i_octal = 0o77;
    let i_binary = 0b1111_0000;
    let i_byte = b'A';
    println!("Integer literals: decimal {}, hex {}, octal {}, binary {}, byte {}", i_decimal, i_hex, i_octal, i_binary, i_byte);
}

fn learn_floating_types() {

    let f32_min: f32 = f32::MIN;
    let f32_max: f32 = f32::MAX;
    println!("f32 range: {} to {}", f32_min, f32_max);

    let f64_min: f64 = f64::MIN;
    let f64_max: f64 = f64::MAX;
    println!("f64 range: {} to {}", f64_min, f64_max);

    let f_decimal = 2.5;
    let f_exponent = 1.5e10;
    println!("Floating-point literals: decimal {}, exponent {}", f_decimal, f_exponent);
}

fn learn_boolean_types() {
    let t: bool = true;
    let f: bool = false;
    println!("Boolean values: t = {}, f = {}", t, f);
}

fn learn_character_types() {
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    println!("Character values: c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}

fn learn_tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The second value of the tuple is: {}", tup.1); 
    let (x, y, z) = tup; 
    println!("The values of x, y, z are: {}, {}, {}", x, y, z);
}

fn learn_array_types() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first element of the array is: {}", arr[0]);
    let arr_default: [i32; 5] = [0; 5];
    println!("The array with default values is: {:?}", arr_default);
}
