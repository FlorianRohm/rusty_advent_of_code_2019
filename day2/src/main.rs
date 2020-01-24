use intcode::{input, run_instruction_set, IntcodeReturnType, Memory};
use itertools::iproduct;

fn main() {
    let original_code = input::get_input_vec("day2");
    let code = get_custom_inputs(&original_code, 12, 2);

    let intcode = run_instruction_set(code);

    println!("Intcode Return: {:?}", intcode);

    let valid_values: Vec<i32> = find_inputs_for(&original_code, 19_690_720)
        .iter()
        .map(|(noun, verb)| 100 * noun + verb)
        .collect();

    println!("valid inputs are: {:?}", valid_values)
}

fn find_inputs_for(memory: &Memory, wanted_output: i32) -> Vec<(i32, i32)> {
    let mut valid_values = vec![];
    for (noun, verb) in iproduct!(0..99, 0..99) {
        let code = get_custom_inputs(&memory, noun, verb);

        let intcode = run_instruction_set(code);

        match intcode {
            IntcodeReturnType::CodeError => {
                continue;
            }
            IntcodeReturnType::IndexError => continue,
            IntcodeReturnType::Finished(state) => {
                let output = state.code[0];
                if output == wanted_output {
                    valid_values.push((noun, verb))
                }
            }
            IntcodeReturnType::Interrupted(_) => {
                unreachable!("day two does not have interrupts");
            }
        }
    }

    valid_values
}

fn get_custom_inputs(memory: &Memory, noun: i32, verb: i32) -> Memory {
    let mut new_memory = memory.clone();

    new_memory[1] = noun;
    new_memory[2] = verb;

    new_memory
}
