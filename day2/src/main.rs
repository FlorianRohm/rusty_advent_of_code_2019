use itertools::iproduct;
use std::fs;
use std::str::FromStr;
use intcode::{run_instruction_set, Memory, IntcodeReturnType};

fn main() {
    let original_code = get_input_vec();
    let code = get_custom_inputs(&original_code, 12, 2);

    let intcode = run_instruction_set(code);

    println!("Intcode Return: {:?}", intcode);

    let valid_values: Vec<usize> = find_inputs_for(&original_code, 19_690_720)
        .iter()
        .map(|(noun, verb)| 100 * noun + verb)
        .collect();

    println!("valid inputs are: {:?}", valid_values)
}

fn find_inputs_for(memory: &Memory, wanted_output: usize) -> Vec<(usize, usize)> {
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
        }
    }

    valid_values
}

fn get_custom_inputs(memory: &Memory, noun: usize, verb: usize) -> Memory {
    let mut new_memory = memory.clone();

    new_memory[1] = noun;
    new_memory[2] = verb;

    new_memory
}

fn get_input_vec() -> Memory {
    let filename = "./day2/resources/input_orig";
    let code: Memory = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|line| usize::from_str(line).expect("Parsing error"))
        .collect();
    code
}
