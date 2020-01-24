use intcode::{input, run_instruction_set_with_input, IntcodeReturnType, Memory};
use permutohedron::Heap;

fn main() {
    let code = input::get_input_vec("day7");

    let mut data = [0, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);

    let (optimal_perm, max_thrust) = heap
        .map(|permutation| (permutation, run_settings(permutation, &code)))
        .max_by(|(_, r1), (_, r2)| r1.cmp(r2))
        .unwrap();

    print!(
        "Maximal thrust is at {}, reached with {:?}",
        max_thrust, optimal_perm
    )
}

fn run_settings(settings: [i32; 5], code: &Memory) -> i32 {
    let return_type = step(settings[0], 0, code.to_owned());
    let next_code = get_output(&return_type);

    let return_type = step(settings[1], next_code, code.to_owned());
    let next_code = get_output(&return_type);

    let return_type = step(settings[2], next_code, code.to_owned());
    let next_code = get_output(&return_type);

    let return_type = step(settings[3], next_code, code.to_owned());
    let next_code = get_output(&return_type);

    let return_type = step(settings[4], next_code, code.to_owned());
    get_output(&return_type)
}

fn step(start_input: i32, second_input: i32, code: Vec<i32>) -> IntcodeReturnType {
    let intcode = run_instruction_set_with_input(code.clone(), start_input);
    return intcode.resume_with_input(second_input);
}

fn get_output(return_type: &IntcodeReturnType) -> i32 {
    if let IntcodeReturnType::Finished(state) = return_type {
        return *state.output.first().unwrap();
    } else {
        panic!("not expected")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod single_run {
        use super::*;
        #[test]
        fn test_run_settings_1() {
            let strength = run_settings(
                [4, 3, 2, 1, 0],
                &vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ],
            );

            assert_eq!(strength, 43210);
        }
        #[test]
        fn test_run_settings_2() {
            let strength = run_settings(
                [0, 1, 2, 3, 4],
                &vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0,
                ],
            );

            assert_eq!(strength, 54321);
        }

        #[test]
        fn test_run_settings_3() {
            let strength = run_settings(
                [1, 0, 4, 3, 2],
                &vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ],
            );

            assert_eq!(strength, 65210);
        }
    }
}
