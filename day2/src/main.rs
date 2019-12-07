use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum IntcodeReturnType {
    CodeError,
    IndexError,
    Finished(IntcodeState),
}


#[derive(Debug, PartialEq)]
struct IntcodeState {
    code: Vec<usize>,
    index: usize,
}

type IntcodeResult = std::result::Result<IntcodeState, IntcodeReturnType>;


fn main() {
    let filename = "./day2/resources/input_edit";

    let code: Vec<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .split(',')
        .map(|line| usize::from_str(line).expect("Parsing error"))
        .collect();

    let intcode = complete_intcode(IntcodeState { code, index: 0 });

    println!("Intcode Return: {:?}", intcode)
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
    let mut code = intcode_state.code;
    let index = intcode_state.index;
    let op_mode = code.get(index).ok_or(IntcodeReturnType::IndexError)?.to_owned();

    if op_mode == 99 { return Err(IntcodeReturnType::Finished(IntcodeState { code, index })) }

    let index_1 = code.get(index + 1).ok_or(IntcodeReturnType::IndexError)?.to_owned();
    let index_2 = code.get(index + 2).ok_or(IntcodeReturnType::IndexError)?.to_owned();

    let target_index = code.get(index + 3).ok_or(IntcodeReturnType::IndexError)?.to_owned();
    code.get(target_index).ok_or(IntcodeReturnType::IndexError)?;

    let operand_1 = code.get(index_1).ok_or(IntcodeReturnType::IndexError)?.to_owned();
    let operand_2 = code.get(index_2).ok_or(IntcodeReturnType::IndexError)?.to_owned();

    let operation_result = match op_mode {
        1 => operand_1 + operand_2,
        2 => operand_1 * operand_2,
        _ => return Err(IntcodeReturnType::CodeError),
    };

    code[target_index] = operation_result;

    Ok(IntcodeState { code, index: index + 4 })
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
            assert_eq!(complete_intcode(IntcodeState { code: vec![1, 0, 0, 0, 0, 34, 4, 5], index: 0 }), IntcodeReturnType::IndexError);
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
