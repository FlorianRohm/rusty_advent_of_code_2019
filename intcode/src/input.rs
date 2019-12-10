use std::fs;
use crate::Memory;
use std::str::FromStr;

pub fn get_input_vec(day: &str) -> Memory {
    let filename = format!("./{}/resources/input", day);
    let code: Memory = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|line| i32::from_str(line.trim()).expect(format!("Parsing error with {}", line).as_ref()))
        .collect();
    code
}
