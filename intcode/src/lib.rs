use crate::ProgramState::{Running, Halted};

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
    input: usize,
}

enum ProgramState {
    Running(OpMode),
    Halted,
}

enum OpMode {
    Add,
    Mul,
    Input,
}

pub type Memory = Vec<usize>;
pub type IntcodeResult = std::result::Result<IntcodeState, IntcodeReturnType>;

impl IntcodeState {
    pub fn from(code: Memory) -> IntcodeState {
        IntcodeState { code, index: 0, input: 0 }
    }
    pub fn from_input(code: Memory, input: usize) -> IntcodeState {
        IntcodeState { code, index: 0, input }
    }
    fn from_all(code: Memory, index: usize, input: usize) -> IntcodeState {
        IntcodeState { code, index, input }
    }
}

impl ProgramState {
    fn from_usize(input: usize) -> Result<Self, IntcodeReturnType> {
        match input {
            1 => Ok(Running(OpMode::Add)),
            2 => Ok(Running(OpMode::Mul)),
            3 => Ok(Running(OpMode::Input)),
            99 => Ok(Halted),
            _ => Err(IntcodeReturnType::CodeError),
        }
    }
}

impl OpMode {
    fn get_index_increase(&self) -> usize {
        match self {
            OpMode::Add => 4,
            OpMode::Mul => 4,
            OpMode::Input => 2
        }
    }

    fn result_index_offset(&self) -> usize {
        match self {
            OpMode::Add => 3,
            OpMode::Mul => 3,
            OpMode::Input => 1
        }
    }
}

pub fn run_instruction_set(memory: Memory) -> IntcodeReturnType {
    complete_intcode(IntcodeState::from(memory))
}

fn complete_intcode(mut intcode_state: IntcodeState) -> IntcodeReturnType {
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
    let op_mode = match ProgramState::from_usize(*code.get(index).ok_or(IntcodeReturnType::IndexError)?)? {
        Running(op_mode) => op_mode,
        Halted => return Err(IntcodeReturnType::Finished(IntcodeState::from_all(code, index, 0))),
    };

    let new_value = match op_mode {
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
        OpMode::Input => {
            intcode_state.input
        }
    };

    let code = try_set_at_index_location(code, index + op_mode.result_index_offset(), new_value)?;

    Ok(IntcodeState::from_all(code, index + op_mode.get_index_increase(), intcode_state.input))
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
                Ok(IntcodeState::from_all(vec![2, 0, 0, 0], 4, 0))
            );
        }

        #[test]
        fn test_intcode_step_mul() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 0])),
                Ok(IntcodeState::from_all(vec![4, 0, 0, 0], 4, 0))
            );
        }

        #[test]
        fn test_intcode_step_add_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 0, 3])),
                Ok(IntcodeState::from_all(vec![1, 0, 0, 2], 4, 0))
            );
        }

        #[test]
        fn test_intcode_step_mul_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 2])),
                Ok(IntcodeState::from_all(vec![2, 0, 4, 2], 4, 0))
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
                Err(IntcodeReturnType::Finished(IntcodeState::from_all(vec![99, 0, 0, 5], 0, 0)))
            );
        }

        #[test]
        fn test_intcode_step_input() {
            assert_eq!(
                intcode_step(IntcodeState::from_input(vec![3, 0], 5)),
                Ok(IntcodeState::from_all(vec![5, 0], 2, 5))
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
                IntcodeReturnType::Finished(IntcodeState::from_all(vec![
                    3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50
                ], 8, 0))
            );

            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 0, 0, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(vec![2, 0, 0, 0, 99], 4, 0))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 3, 0, 3, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(vec![2, 3, 0, 6, 99], 4, 0))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 4, 4, 5, 99, 0])),
                IntcodeReturnType::Finished(IntcodeState::from_all(vec![2, 4, 4, 5, 99, 9801], 4, 0))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], 8, 0))
            );
        }
    }
}
