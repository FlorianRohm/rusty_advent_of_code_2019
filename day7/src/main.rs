use intcode::{input, run_instruction_set_with_input};

fn main() {
    let code = input::get_input_vec("day7");

    let intcode = run_instruction_set_with_input(code.clone(), 1);
}
