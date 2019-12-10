#[derive(Debug, PartialEq)]
pub enum IntcodeReturnType {
    CodeError,
    IndexError,
    Finished(IntcodeState),
}

#[derive(Debug, PartialEq)]
pub struct IntcodeState {
    pub code: Memory,
    index: usize,
}

enum OpMode {
    Add,
    Mul,
    Terminate,
}

pub type Memory = Vec<usize>;
pub type IntcodeResult = std::result::Result<IntcodeState, IntcodeReturnType>;

impl IntcodeState {
    pub fn from(code: Memory) -> IntcodeState {
        IntcodeState { code, index: 0 }
    }
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
            _ => Err(IntcodeReturnType::CodeError),
        }
    }
}

pub fn complete_intcode(mut intcode_state: IntcodeState) -> IntcodeReturnType {
    loop {
        intcode_state = match intcode_step(intcode_state) {
            Ok(t) => t,
            Err(return_type) => return return_type,
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
        }
        OpMode::Mul => {
            let operand_1 = get_value_at_index_location(&code, index + 1)?;
            let operand_2 = get_value_at_index_location(&code, index + 2)?;
            operand_1 * operand_2
        }
    };

    let code = try_set_at_index_location(code, index + 3, new_value)?;

    Ok(IntcodeState {
        code,
        index: index + op_mode.to_index_increase(),
    })
}

fn get_value_at_index_location(code: &Memory, index: usize) -> Result<usize, IntcodeReturnType> {
    let index_1 = code
        .get(index)
        .ok_or(IntcodeReturnType::IndexError)?
        .to_owned();
    Ok(code
        .get(index_1)
        .ok_or(IntcodeReturnType::IndexError)?
        .to_owned())
}

fn try_set_at_index_location(
    mut code: Memory,
    index: usize,
    value: usize,
) -> Result<Memory, IntcodeReturnType> {
    let target_index = code
        .get(index)
        .ok_or(IntcodeReturnType::IndexError)?
        .to_owned();
    code.get(target_index)
        .ok_or(IntcodeReturnType::IndexError)?;
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
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 0, 0])),
                Ok(IntcodeState::from(vec![2, 0, 0, 0]))
            );
        }

        #[test]
        fn test_intcode_step_mul() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 0])),
                Ok(IntcodeState::from(vec![4, 0, 0, 0]))
            );
        }

        #[test]
        fn test_intcode_step_add_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 0, 3])),
                Ok(IntcodeState::from(vec![1, 0, 0, 2]))
            );
        }

        #[test]
        fn test_intcode_step_mul_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 2])),
                Ok(IntcodeState::from(vec![2, 0, 4, 2]))
            );
        }

        #[test]
        fn test_intcode_step_err_index_1() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 5, 0, 1])),
                Err(IntcodeReturnType::IndexError)
            );
        }

        #[test]
        fn test_intcode_step_err_index_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 5, 1])),
                Err(IntcodeReturnType::IndexError)
            );
        }

        #[test]
        fn test_intcode_step_err_index_3() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 0, 5])),
                Err(IntcodeReturnType::IndexError)
            );
        }

        #[test]
        fn test_intcode_return() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![99, 0, 0, 5])),
                Err(IntcodeReturnType::Finished(IntcodeState::from(vec![
                    99, 0, 0, 5
                ])))
            );
        }
    }

    mod test_complete {
        use super::*;

        #[test]
        fn test_intcode_index_error_1() {
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 0, 0, 0])),
                IntcodeReturnType::IndexError
            );
        }

        #[test]
        fn test_intcode_index_error_2() {
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 0, 0, 0, 1, 34, 4, 5])),
                IntcodeReturnType::IndexError
            );
        }

        #[test]
        fn test_intcode_website() {
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![
                    1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50
                ])),
                IntcodeReturnType::Finished(IntcodeState::from(vec![
                    3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50
                ]))
            );

            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 0, 0, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from(vec![2, 0, 0, 0, 99]))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 3, 0, 3, 99])),
                IntcodeReturnType::Finished(IntcodeState::from(vec![2, 3, 0, 6, 99]))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 4, 4, 5, 99, 0])),
                IntcodeReturnType::Finished(IntcodeState::from(vec![2, 4, 4, 5, 99, 9801]))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from(vec![30, 1, 1, 4, 2, 5, 6, 0, 99]))
            );
        }
    }
}
