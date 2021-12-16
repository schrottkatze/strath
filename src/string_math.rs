fn multiply_strings(str: String, multiplier: f32) -> String {
    let mut r: String = String::from(str);
    let rest = multiplier - (multiplier as i32) as f32;
    let slice = &r.clone()[0..(r.len() as f32 * rest) as usize];

    r = r.repeat(multiplier as usize);
    r += slice;
    r
}
