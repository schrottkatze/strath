use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Couldnt read file:");
    println!("{:?}", multiply_strings(file, 3.5));
}

fn multiply_strings(str: String, multiplier: f32) -> String {
    let mut r: String = String::from(str);
    let mult_float = multiplier - (multiplier as i32) as f32;
    let slice = &r.clone()[0..(r.len() as f32 * mult_float) as usize];

    r = r.repeat(multiplier as usize);
    r += slice;
    r
}
