use std::fs;
use std::str::FromStr;

fn main() {
    let filename = "./day1/resources/input";
    println!("In file {}", filename);

    let fuel_requirements: i32 = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| i32::from_str(line).expect("Parsing error"))
        .map(get_fuel)
        .sum();

    println!("Fuel Requirements:\n{}", fuel_requirements);
}

fn get_fuel(i: i32) -> i32 {
    i / 3 - 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel() {
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(14), 2);
        assert_eq!(get_fuel(1969), 654);
        assert_eq!(get_fuel(100756), 33583);
    }
}
