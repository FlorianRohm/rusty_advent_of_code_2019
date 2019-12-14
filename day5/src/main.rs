use intcode::{input, run_instruction_set_with_input};

fn main() {
    let code = input::get_input_vec("day5");

    let intcode = run_instruction_set_with_input(code.clone(), 1);

    println!("Intcode Return: {:?}", intcode);

    let intcode = run_instruction_set_with_input(code.clone(), 5);

    println!("Intcode Return: {:?}", intcode);
}
