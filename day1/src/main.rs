use std::fs;
use std::str::FromStr;

fn main() {
    let filename = "./day1/resources/input";
    println!("In file {}", filename);

    let (fuel_requirements_initial, fuel_requirements_total): (i32, i32) =
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file")
            .lines()
            .map(|line| i32::from_str(line).expect("Parsing error"))
            .map(|module_mass| {
                (
                    get_fuel_requirement_for_mass(module_mass),
                    get_complete_fuel_requirement_for_mass(module_mass),
                )
            })
            .fold((0, 0), |acc: (i32, i32), next: (i32, i32)| {
                (acc.0 + next.0, acc.1 + next.1)
            });

    println!("Fuel Requirements initial: {}", fuel_requirements_initial);
    println!("Fuel Requirements total:   {}", fuel_requirements_total);
}

fn get_fuel_requirement_for_mass(i: i32) -> i32 {
    i / 3 - 2
}

fn get_complete_fuel_requirement_for_mass(initial_mass: i32) -> i32 {
    let mut additional_fuel = get_fuel_requirement_for_mass(initial_mass);
    let mut fuel_for_fuel = 0;

    while additional_fuel >= 0 {
        fuel_for_fuel -= -additional_fuel;
        additional_fuel = get_fuel_requirement_for_mass(additional_fuel)
    }

    fuel_for_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel_requirement_for_mass() {
        assert_eq!(get_fuel_requirement_for_mass(12), 2);
        assert_eq!(get_fuel_requirement_for_mass(14), 2);
        assert_eq!(get_fuel_requirement_for_mass(1969), 654);
        assert_eq!(get_fuel_requirement_for_mass(100756), 33583);
    }

    #[test]
    fn test_get_complete_fuel_requirement_for_mass() {
        assert_eq!(get_complete_fuel_requirement_for_mass(1969), 966);
    }
}
