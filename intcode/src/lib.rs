use crate::IntcodeReturnType::CodeError;
use crate::ParamMode::{Immediate, Position};
use crate::ProgramState::{Halted, Running};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub enum IntcodeReturnType {
    CodeError,
    IndexError,
    Finished(IntcodeState),
}

#[derive(Debug, PartialEq, Default)]
pub struct IntcodeState {
    pub code: Memory,
    index: usize,
    input: i32,
    pub output: Vec<i32>,
}

enum ProgramState {
    Running(OpMode),
    Halted,
}

enum ParamMode {
    Position,
    Immediate,
}

enum OpMode {
    Add(ParamMode, ParamMode),
    Mul(ParamMode, ParamMode),
    Input,
    Output(ParamMode),
}

pub type Memory = Vec<i32>;
pub type IntcodeResult = std::result::Result<IntcodeState, IntcodeReturnType>;

impl IntcodeState {
    pub fn from(code: Memory) -> IntcodeState {
        IntcodeState {
            code,
            ..IntcodeState::default()
        }
    }
    pub fn from_input(code: Memory, input: i32) -> IntcodeState {
        IntcodeState {
            code,
            input,
            ..IntcodeState::default()
        }
    }
    fn from_all(code: Memory, index: usize, input: i32, output: Vec<i32>) -> IntcodeState {
        IntcodeState {
            code,
            index,
            input,
            output,
        }
    }
}

impl TryFrom<usize> for ParamMode {
    type Error = IntcodeReturnType;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Position),
            1 => Ok(Immediate),
            _ => Err(CodeError),
        }
    }
}

impl ProgramState {
    fn from_memory_location(input: i32) -> Result<Self, IntcodeReturnType> {
        assert!(input <= 99999);
        let mut n: usize = input.try_into().map_err(|_| IntcodeReturnType::CodeError)?;
        let op_mode = n % 100;
        n = n / 100;

        let first_param = ParamMode::try_from(n % 10)?;
        n = n / 10;
        let second_param = ParamMode::try_from(n % 10)?;
        n = n / 10;
        let third_param = ParamMode::try_from(n % 10)?;
        n = n / 10;

        match op_mode {
            1 => Ok(Running(OpMode::Add(first_param, second_param))),
            2 => Ok(Running(OpMode::Mul(first_param, second_param))),
            3 => Ok(Running(OpMode::Input)),
            4 => Ok(Running(OpMode::Output(first_param))),
            99 => Ok(Halted),
            _ => Err(IntcodeReturnType::CodeError),
        }
    }
}

impl OpMode {
    fn get_index_increase(&self) -> usize {
        match self {
            OpMode::Add(_, _) => 4,
            OpMode::Mul(_, _) => 4,
            OpMode::Input => 2,
            OpMode::Output(_) => 2,
        }
    }

    fn result_index_offset(&self) -> usize {
        match self {
            OpMode::Add(_, _) => 3,
            OpMode::Mul(_, _) => 3,
            OpMode::Input => 1,
            OpMode::Output(_) => 1,
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

fn intcode_step(mut intcode_state: IntcodeState) -> IntcodeResult {
    let index = intcode_state.index;
    let instruction_field = get_index_value(&intcode_state.code, index)?;

    let op_mode = match ProgramState::from_memory_location(instruction_field)? {
        Running(op_mode) => op_mode,
        Halted => return Err(IntcodeReturnType::Finished(intcode_state)),
    };

    let new_state = process_op_mode(intcode_state, op_mode)?;

    Ok(new_state)
}

fn process_op_mode(mut intcode_state: IntcodeState, op_mode: OpMode) -> IntcodeResult {
    let index = intcode_state.index;

    let mut new_state = match op_mode {
        OpMode::Add(ref mode_1, ref mode_2) => {
            let operand_1 = get_value_at_index_location(&intcode_state.code, index + 1, mode_1)?;
            let operand_2 = get_value_at_index_location(&intcode_state.code, index + 2, mode_2)?;
            intcode_state.code = try_set_at_index_location(
                intcode_state.code,
                index + op_mode.result_index_offset(),
                operand_1 + operand_2,
            )?;

            intcode_state
        }
        OpMode::Mul(ref mode_1, ref mode_2)  => {
            let operand_1 = get_value_at_index_location(&intcode_state.code, index + 1, mode_1)?;
            let operand_2 = get_value_at_index_location(&intcode_state.code, index + 2, mode_2)?;

            intcode_state.code = try_set_at_index_location(
                intcode_state.code,
                index + op_mode.result_index_offset(),
                operand_1 * operand_2,
            )?;

            intcode_state
        }
        OpMode::Input => {
            intcode_state.code = try_set_at_index_location(
                intcode_state.code,
                index + op_mode.result_index_offset(),
                intcode_state.input,
            )?;

            intcode_state
        }
        OpMode::Output(ref mode) => {
            let output = get_value_at_index_location(&intcode_state.code, index + 1, mode)?;

            intcode_state.output.push(output);
            intcode_state
        }
    };
    new_state.index += op_mode.get_index_increase();
    Ok(new_state)
}

fn get_index_value(code: &Memory, index: usize) -> Result<i32, IntcodeReturnType> {
    Ok(code
        .get(index)
        .ok_or(IntcodeReturnType::IndexError)?
        .to_owned())
}

fn get_value_at_index_location(
    code: &Memory,
    index: usize,
    mode: &ParamMode,
) -> Result<i32, IntcodeReturnType> {
    let index_value = get_index_value(code, index)?;
    match mode {
        Immediate => Ok(index_value as i32),
        Position => {
            let i: usize = index_value
                .try_into()
                .map_err(|_| IntcodeReturnType::IndexError)?;

            Ok(code.get(i).ok_or(IntcodeReturnType::IndexError)?.to_owned())
        }
    }
}

fn try_set_at_index_location(
    mut code: Memory,
    index: usize,
    value: i32,
) -> Result<Memory, IntcodeReturnType> {
    let target_index: usize = code
        .get(index)
        .ok_or(IntcodeReturnType::IndexError)?
        .to_owned()
        .try_into()
        .map_err(|_| IntcodeReturnType::IndexError)?;
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
                Ok(IntcodeState::from_all(vec![2, 0, 0, 0], 4, 0, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_mul() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 0])),
                Ok(IntcodeState::from_all(vec![4, 0, 0, 0], 4, 0, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_add_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1, 0, 0, 3])),
                Ok(IntcodeState::from_all(vec![1, 0, 0, 2], 4, 0, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_mul_2() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![2, 0, 0, 2])),
                Ok(IntcodeState::from_all(vec![2, 0, 4, 2], 4, 0, vec![]))
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
                Err(IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![99, 0, 0, 5],
                    0,
                    0,
                    vec![],
                )))
            );
        }

        #[test]
        fn test_intcode_step_input() {
            assert_eq!(
                intcode_step(IntcodeState::from_input(vec![3, 0], 5)),
                Ok(IntcodeState::from_all(vec![5, 0], 2, 5, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_output() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![4, 1])),
                Ok(IntcodeState::from_all(vec![4, 1], 2, 0, vec![1]))
            );

            assert_eq!(
                intcode_step(IntcodeState::from(vec![4, 0])),
                Ok(IntcodeState::from_all(vec![4, 0], 2, 0, vec![4]))
            );
        }

        #[test]
        fn test_intcode_step_parameter_mode_mul() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1002, 4, 3, 4, 33])),
                Ok(IntcodeState::from_all(
                    vec![1002, 4, 3, 4, 99],
                    4,
                    0,
                    vec![]
                ))
            );
        }

        #[test]
        fn test_intcode_step_negative() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1101,100,-1,4,0])),
                Ok(IntcodeState::from_all(
                    vec![1101,100,-1,4,99],
                    4,
                    0,
                    vec![]
                ))
            );
        }

        #[test]
        fn test_intcode_step_parameter_mode_add() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1101, 4, 3, 4, 33])),
                Ok(IntcodeState::from_all(
                    vec![1101, 4, 3, 4, 7],
                    4,
                    0,
                    vec![]
                ))
            );
        }

        #[test]
        fn test_intcode_step_parameter_mode_out() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![104, 55, 3, 4, 33])),
                Ok(IntcodeState::from_all(
                    vec![104, 55, 3, 4, 33],
                    2,
                    0,
                    vec![55]
                ))
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
                IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                    8,
                    0,
                    vec![],
                ))
            );

            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 0, 0, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![2, 0, 0, 0, 99],
                    4,
                    0,
                    vec![],
                ))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 3, 0, 3, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![2, 3, 0, 6, 99],
                    4,
                    0,
                    vec![],
                ))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![2, 4, 4, 5, 99, 0])),
                IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![2, 4, 4, 5, 99, 9801],
                    4,
                    0,
                    vec![],
                ))
            );
            assert_eq!(
                complete_intcode(IntcodeState::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])),
                IntcodeReturnType::Finished(IntcodeState::from_all(
                    vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
                    8,
                    0,
                    vec![],
                ))
            );
        }
    }
}
