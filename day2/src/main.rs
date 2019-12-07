use std::fs;
use std::str::FromStr;
use itertools::iproduct;

#[derive(Debug, PartialEq)]
enum IntcodeReturnType {
    CodeError,
    IndexError,
    Finished(IntcodeState),
}

type Memory = Vec<usize>;

#[derive(Debug, PartialEq)]
struct IntcodeState {
    code: Memory,
    index: usize,
}

impl IntcodeState {
    fn from(code: Memory) -> IntcodeState {
        IntcodeState { code, index: 0 }
    }
}

enum OpMode {
    Add,
    Mul,
    Terminate,
}

impl OpMode {
    fn to_index_increase(&self) -> usize {
        match self {
            OpMode::Add => 4,
            OpMode::Mul => 4,
            OpMode::Terminate => 1,
        }
    }

    fn from_usize(input: usize) -> Result<Self, IntcodeReturnType> {
        match input {
            1 => Ok(OpMode::Add),
            2 => Ok(OpMode::Mul),
            99 => Ok(OpMode::Terminate),
            _ => Err(IntcodeReturnType::CodeError)
        }
    }
}

type IntcodeResult = std::result::Result<IntcodeState, IntcodeReturnType>;

fn main() {
    let original_code = get_input_vec();
    let code = get_custom_inputs(&original_code, 12, 2);

    let intcode = complete_intcode(IntcodeState { code, index: 0 });

    println!("Intcode Return: {:?}", intcode);

    let valid_values: Vec<usize> = find_inputs_for(&original_code, 19_690_720).iter()
        .map(|(noun, verb)| 100 * noun + verb).collect();

    println!("valid inputs are: {:?}", valid_values)
}

fn find_inputs_for(memory: &Memory, wanted_output: usize) -> Vec<(usize, usize)> {
    let mut valid_values = vec![];
    for (noun, verb) in iproduct!(0..99, 0..99) {
        let code = get_custom_inputs(&memory, noun, verb);

        let intcode = complete_intcode(IntcodeState::from(code));

        match intcode {
            IntcodeReturnType::CodeError => { continue; },
            IntcodeReturnType::IndexError => { continue },
            IntcodeReturnType::Finished(state) => {
                let output = state.code[0];
                if output == wanted_output {
                    valid_values.push((noun, verb))
                }
            },
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


fn complete_intcode(mut intcode_state: IntcodeState) -> IntcodeReturnType {
    loop {
        intcode_state = match intcode_step(intcode_state) {
            Ok(t) => t,
            Err(return_type) => return return_type
        };
    }
}

fn intcode_step(intcode_state: IntcodeState) -> IntcodeResult {
    let code = intcode_state.code;
    let index = intcode_state.index;
    let op_mode = OpMode::from_usize(*code.get(index).ok_or(IntcodeReturnType::IndexError)?)?;

    let new_value = match op_mode {
        OpMode::Terminate => return Err(IntcodeReturnType::Finished(IntcodeState { code, index })),
        OpMode::Add => {
            let operand_1 = get_value_at_index_location(&code, index + 1)?;
            let operand_2 = get_value_at_index_location(&code, index + 2)?;
            operand_1 + operand_2
        },
        OpMode::Mul => {
            let operand_1 = get_value_at_index_location(&code, index + 1)?;
            let operand_2 = get_value_at_index_location(&code, index + 2)?;
            operand_1 * operand_2
        }
    };

    let code = try_set_at_index_location(code, index + 3, new_value)?;

    Ok(IntcodeState { code, index: index + op_mode.to_index_increase() })
}

fn get_value_at_index_location(code: &Memory, index: usize) -> Result<usize, IntcodeReturnType> {
    let index_1 = code.get(index).ok_or(IntcodeReturnType::IndexError)?.to_owned();
    Ok(code.get(index_1).ok_or(IntcodeReturnType::IndexError)?.to_owned())
}

fn try_set_at_index_location(mut code: Memory, index: usize, value: usize) -> Result<Memory, IntcodeReturnType> {
    let target_index = code.get(index).ok_or(IntcodeReturnType::IndexError)?.to_owned();
    code.get(target_index).ok_or(IntcodeReturnType::IndexError)?;
    code[target_index] = value;

    Ok(code)
}


#[cfg(test)]
mod tests {
    use super::*;

    mod test_step {
        use super::*;

        #[test]
        fn test_intcode_step_add() {
            assert_eq!(intcode_step(IntcodeState { code: vec![1, 0, 0, 0], index: 0 }), Ok(IntcodeState { code: vec![2, 0, 0, 0], index: 4 }));
        }

        #[test]
        fn test_intcode_step_mul() {
            assert_eq!(intcode_step(IntcodeState { code: vec![2, 0, 0, 0], index: 0 }), Ok(IntcodeState { code: vec![4, 0, 0, 0], index: 4 }));
        }

        #[test]
        fn test_intcode_step_add_2() {
            assert_eq!(intcode_step(IntcodeState { code: vec![1, 0, 0, 3], index: 0 }), Ok(IntcodeState { code: vec![1, 0, 0, 2], index: 4 }));
        }

        #[test]
        fn test_intcode_step_mul_2() {
            assert_eq!(intcode_step(IntcodeState { code: vec![2, 0, 0, 2], index: 0 }), Ok(IntcodeState { code: vec![2, 0, 4, 2], index: 4 }));
        }


        #[test]
        fn test_intcode_step_err_index_1() {
            assert_eq!(intcode_step(IntcodeState { code: vec![1, 5, 0, 1], index: 0 }), Err(IntcodeReturnType::IndexError));
        }

        #[test]
        fn test_intcode_step_err_index_2() {
            assert_eq!(intcode_step(IntcodeState { code: vec![1, 0, 5, 1], index: 0 }), Err(IntcodeReturnType::IndexError));
        }

        #[test]
        fn test_intcode_step_err_index_3() {
            assert_eq!(intcode_step(IntcodeState { code: vec![1, 0, 0, 5], index: 0 }), Err(IntcodeReturnType::IndexError));
        }

        #[test]
        fn test_intcode_return() {
            assert_eq!(intcode_step(IntcodeState { code: vec![99, 0, 0, 5], index: 0 }), Err(IntcodeReturnType::Finished(IntcodeState { code: vec![99, 0, 0, 5], index: 0 })));
        }
    }

    mod test_complete {
        use super::*;

        #[test]
        fn test_intcode_index_error_1() {
            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 0, 0, 0], index: 0 }), IntcodeReturnType::IndexError);
        }

        #[test]
        fn test_intcode_index_error_2() {
            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 0, 0, 0, 1, 34, 4, 5], index: 0 }), IntcodeReturnType::IndexError);
        }


        #[test]
        fn test_intcode_website() {
            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], index: 0 }),
                       IntcodeReturnType::Finished(IntcodeState { code: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], index: 8 }));

            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 0, 0, 0, 99], index: 0 }), IntcodeReturnType::Finished(IntcodeState { code: vec![2, 0, 0, 0, 99], index: 4 }));
            assert_eq!(complete_intcode(IntcodeState { code: vec![2, 3, 0, 3, 99], index: 0 }), IntcodeReturnType::Finished(IntcodeState { code: vec![2, 3, 0, 6, 99], index: 4 }));
            assert_eq!(complete_intcode(IntcodeState { code: vec![2, 4, 4, 5, 99, 0], index: 0 }), IntcodeReturnType::Finished(IntcodeState { code: vec![2, 4, 4, 5, 99, 9801], index: 4 }));
            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 1, 1, 4, 99, 5, 6, 0, 99], index: 0 }), IntcodeReturnType::Finished(IntcodeState { code: vec![30, 1, 1, 4, 2, 5, 6, 0, 99], index: 8 }));
        }
    }
}
