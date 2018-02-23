use std::{i8, i16, i32, i64, f32, f64, isize, usize, u8, u16, u32, u64, char};

fn main() {

//integers

let i8_min = i8::MIN;
let i8_max = i8::MAX;

let i16_min = i16::MIN;
let i16_max = i16::MAX;

let i32_min = i32::MIN;
let i32_max = i32::MAX;

let i64_min = i64::MIN;
let i64_max = i64::MAX;

let u8_min = u8::MIN;
let u8_max = u8::MAX;

let u16_min = u16::MIN;
let u16_max = u16::MAX;

let u32_min = u32::MIN;
let u32_max = u32::MAX;

let u64_min = u64::MIN;
let u64_max = u64::MAX;


let f32_min = f32::MIN; 
let f32_max = f32::MAX;

let f64_min = f64::MIN;
let f64_max = f64::MAX;

let isize_min = isize::MIN;
let isize_max = isize::MAX;

let usize_min = usize::MIN;
let usize_max = usize::MAX;


let a_char: char = '╥';
println!("i8 min {}, max {}", i8_min, i8_max);
println!("\ni16 min {}, max {}", i16_min, i16_max);
println!("\ni32 min {}, max {}", i32_min, i32_max);
println!("\ni64 min {}, max {}", i64_min, i64_max);
println!("\nf32 min {}, max {}", f32_min, f32_max);
println!("\nf64 min {}, max {}", f64_min, f64_max);
println!("\nisize min {}, max {}", isize_min, isize_max);
println!("\nusize min {}, max {}", usize_min, usize_max);
println!("u8 min {}, max {}", u8_min, u8_max);
println!("\nu16 min {}, max {}", u16_min, u16_max);
println!("\nu32 min {}, max {}", u32_min, u32_max);
println!("\nu64 min {}, max {}", u64_min, u64_max);
//println!("i16 min {}, max {}", i16_min, i16_max);
//println!("i16 min {}, max {}", i16_min, i16_max);
println!("a_char {}", a_char);

//Convention Decimal, Hexa, Octal, Binary, Byte (u8 only)
let a_number_to_convert:u16 = 1234;
//println!("Decimal of {:d}",a_number_to_convert);
println!("Hexa of {} is {:x}",a_number_to_convert ,a_number_to_convert);
println!("octal of {} is {:o}",a_number_to_convert,a_number_to_convert);
println!("Binary of {} is {:b}",a_number_to_convert,a_number_to_convert);

//declaring multiple values
let (f_name, l_name, age) = ("Suryaprakash", "Rao", 30);
println!("My name is {0} {1} and I'm {2} old", f_name, l_name, age);

let is_it_true: bool = true;
let x_char: char = 'x';
println!("Is it {0} that {1} is {0}", is_it_true,x_char);

//formatting floats
println!("{:.2}", 1.234);
//formatting texts with named arguments
println!("{ten:>ws$}", ten = 10, ws = 5);
println!("{ten:>0ws$}", ten = 10, ws = 5);

}
