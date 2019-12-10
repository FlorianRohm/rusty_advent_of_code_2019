use crate::IntcodeReturnType::CodeError;
use crate::ParamMode::{Immediate, Position};
use crate::ProgramState::{Halted, Running};
use std::convert::{TryFrom, TryInto};

pub mod input;

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
    JumpIfTrue(ParamMode, ParamMode),
    JumpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode),
    Equals(ParamMode, ParamMode),
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
trait TryToUsize {
    fn to_usize(&self) -> Result<usize, IntcodeReturnType>;
}

impl TryToUsize for i32 {
    fn to_usize(&self) -> Result<usize, IntcodeReturnType> {
        self.clone()
            .try_into()
            .map_err(|_| IntcodeReturnType::IndexError)
    }
}

impl ProgramState {
    fn from_memory_location(input: i32) -> Result<Self, IntcodeReturnType> {
        use OpMode::*;

        assert!(input <= 99999);
        let mut n: usize = input.try_into().map_err(|_| IntcodeReturnType::CodeError)?;
        let op_mode = n % 100;
        n /= 100;

        let first_param = ParamMode::try_from(n % 10)?;
        n /= 10;
        let second_param = ParamMode::try_from(n % 10)?;
        n /= 10;
        let _third_param = ParamMode::try_from(n % 10)?;

        match op_mode {
            1 => Ok(Running(Add(first_param, second_param))),
            2 => Ok(Running(Mul(first_param, second_param))),
            3 => Ok(Running(Input)),
            4 => Ok(Running(Output(first_param))),
            5 => Ok(Running(JumpIfTrue(first_param, second_param))),
            6 => Ok(Running(JumpIfFalse(first_param, second_param))),
            7 => Ok(Running(LessThan(first_param, second_param))),
            8 => Ok(Running(Equals(first_param, second_param))),
            99 => Ok(Halted),
            _ => Err(IntcodeReturnType::CodeError),
        }
    }
}

pub fn run_instruction_set(memory: Memory) -> IntcodeReturnType {
    complete_intcode(IntcodeState::from(memory))
}

pub fn run_instruction_set_with_input(memory: Memory, input: i32) -> IntcodeReturnType {
    complete_intcode(IntcodeState::from_input(memory, input))
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

    let new_state = match op_mode {
        OpMode::Add(mode_1, mode_2) => {
            op_modes_3_inputs(intcode_state, mode_1, mode_2, |a, b| a+b)?
        }
        OpMode::Mul(mode_1, mode_2) => {
            op_modes_3_inputs(intcode_state, mode_1, mode_2, |a, b| a*b)?
        }
        OpMode::Input => {
            intcode_state.code =
                try_set_at_index_location(intcode_state.code, index + 1, intcode_state.input)?;
            intcode_state.index += 2;

            intcode_state
        }
        OpMode::Output(mode) => {
            let output = get_value_at_index_location(&intcode_state.code, index + 1, &mode)?;

            intcode_state.output.push(output);
            intcode_state.index += 2;

            intcode_state
        }

        OpMode::JumpIfTrue(mode_1, mode_2) => {
            match get_value_at_index_location(&intcode_state.code, index + 1, &mode_1)? {
                0 => intcode_state.index += 3,
                _ => {
                    let target =
                        get_value_at_index_location(&intcode_state.code, index + 2, &mode_2)?;
                    intcode_state.index = target.to_usize()?;
                }
            };
            intcode_state
        }

        OpMode::JumpIfFalse(mode_1, mode_2) => {
            match get_value_at_index_location(&intcode_state.code, index + 1, &mode_1)? {
                0 => {
                    let target =
                        get_value_at_index_location(&intcode_state.code, index + 2, &mode_2)?;
                    intcode_state.index = target.to_usize()?;
                }
                _ => intcode_state.index += 3,
            };

            intcode_state
        }
        OpMode::LessThan(mode_1, mode_2) => {
            op_modes_3_inputs(intcode_state, mode_1, mode_2, |a, b| if a < b { 1 } else { 0 })?
        }
        OpMode::Equals(mode_1, mode_2) => {
            op_modes_3_inputs(intcode_state, mode_1, mode_2, |a, b| if a == b { 1 } else { 0 })?
        }
    };

    Ok(new_state)
}

fn op_modes_3_inputs(mut intcode_state: IntcodeState, mode_1: ParamMode, mode_2: ParamMode, operation: impl Fn(i32, i32) -> i32) -> IntcodeResult {
    let index = intcode_state.index;
    let operand_1 = get_value_at_index_location(&intcode_state.code, index + 1, &mode_1)?;
    let operand_2 = get_value_at_index_location(&intcode_state.code, index + 2, &mode_2)?;

    intcode_state.code = try_set_at_index_location(
        intcode_state.code,
        index + 3,
        operation(operand_1, operand_2),
    )?;
    intcode_state.index += 4;

    Ok(intcode_state)
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
            let i: usize = index_value.to_usize()?;

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
        .to_usize()?;
    code.get(target_index)
        .ok_or(IntcodeReturnType::IndexError)?;
    code[target_index] = value;

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    impl IntcodeState {
        fn from_all(code: Memory, index: usize, input: i32, output: Vec<i32>) -> IntcodeState {
            IntcodeState {
                code,
                index,
                input,
                output,
            }
        }
    }


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
                intcode_step(IntcodeState::from(vec![1101, 100, -1, 4, 0])),
                Ok(IntcodeState::from_all(
                    vec![1101, 100, -1, 4, 99],
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
                Ok(IntcodeState::from_all(vec![1101, 4, 3, 4, 7], 4, 0, vec![]))
            );
        }


        #[test]
        fn test_intcode_step_jump_if_not_0_ok() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1105, 1, 5, 4, 33])),
                Ok(IntcodeState::from_all(vec![1105, 1, 5, 4, 33], 5, 0, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_jump_if_not_0_not() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1105, 0, 3, 4, 33])),
                Ok(IntcodeState::from_all(vec![1105, 0, 3, 4, 33], 3, 0, vec![]))
            );
        }


        #[test]
        fn test_intcode_step_jump_if_0_not() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1106, 1, 3, 4, 33])),
                Ok(IntcodeState::from_all(vec![1106, 1, 3, 4, 33], 3, 0, vec![]))
            );
        }

        #[test]
        fn test_intcode_step_jump_if_0_ok() {
            assert_eq!(
                intcode_step(IntcodeState::from(vec![1106, 0, 5, 4, 33])),
                Ok(IntcodeState::from_all(vec![1106, 0, 5, 4, 33], 5, 0, vec![]))
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

        #[test]
        fn test_intcodes_day5_equals() {
            // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
            let input_equal_8 = || vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_equal_8(), 8)),
                vec![1],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_equal_8(), 9)),
                vec![0],
            );
            // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
            let input_less_than_8 = || vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_less_than_8(), 9)),
                vec![0],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_less_than_8(), 5)),
                vec![1],
            );

            // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
            let input_equal_8 = || vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_equal_8(), 8)),
                vec![1],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_equal_8(), 9)),
                vec![0],
            );
            // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
            let input_less_than_8 = || vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_less_than_8(), 9)),
                vec![0],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_less_than_8(), 5)),
                vec![1],
            );
        }

        #[test]
        fn test_intcodes_day5_jumps() {
            // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
            let input = || vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
            let input_2 = || vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

            test_for_output(
                complete_intcode(IntcodeState::from_input(input(), 0)),
                vec![0],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_2(), 0)),
                vec![0],
            );


            test_for_output(
                complete_intcode(IntcodeState::from_input(input(), 5)),
                vec![1],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input_2(), 7)),
                vec![1],
            );
        }

        #[test]
        fn test_intcodes_day5_big() {
            // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
            let input = || vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

            test_for_output(
                complete_intcode(IntcodeState::from_input(input(), 0)),
                vec![999],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input(), 8)),
                vec![1000],
            );
            test_for_output(
                complete_intcode(IntcodeState::from_input(input(), 9)),
                vec![1001],
            );
        }

        fn test_for_output(return_type: IntcodeReturnType, output: Vec<i32>) {
            if let IntcodeReturnType::Finished(state) = return_type {
                assert_eq!(state.output, output)
            } else {
                assert!(false, format!("wrong enum variant {:?}", return_type))
            }
        }
    }
}
