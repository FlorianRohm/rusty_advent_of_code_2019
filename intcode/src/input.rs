use crate::Memory;
use std::fs;
use std::str::FromStr;

pub fn get_input_vec(day: &str) -> Memory {
    let filename = format!("./{}/resources/input", day);
    let code: Memory = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|line| {
            i64::from_str(line.trim()).unwrap_or_else(|_| panic!("Parsing error with {}", line))
        })
        .collect();
    code
}
