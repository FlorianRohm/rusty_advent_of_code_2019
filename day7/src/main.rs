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
    );

    let mut data = [5,6,7,8,9];
    let heap = Heap::new(&mut data);

    let (optimal_perm_amp, max_thrust_amp) = heap
        .map(|permutation| (permutation, run_settings_until_halt(permutation, &code)))
        .max_by(|(_, r1), (_, r2)| r1.cmp(r2))
        .unwrap();

    print!(
        "Maximal amplified thrust is at {}, reached with {:?}",
        max_thrust_amp, optimal_perm_amp
    );
}

fn run_settings(settings: [i64; 5], code: &Memory) -> i64 {
    let return_type = step(settings[0], 0, code.to_owned());
    let (next_code, _) = get_output(return_type);

    let return_type = step(settings[1], next_code, code.to_owned());
    let (next_code, _) = get_output(return_type);

    let return_type = step(settings[2], next_code, code.to_owned());
    let (next_code, _) = get_output(return_type);

    let return_type = step(settings[3], next_code, code.to_owned());
    let (next_code, _) = get_output(return_type);

    let return_type = step(settings[4], next_code, code.to_owned());
    get_output(return_type).0
}


fn run_settings_until_halt(settings: [i64; 5], code: &Memory) -> i64 {
    let mut first_input = 0;
    let mut amp_0_code = code.to_owned();
    let mut amp_1_code = code.to_owned();
    let mut amp_2_code = code.to_owned();
    let mut amp_3_code = code.to_owned();
    let mut amp_4_code = code.to_owned();

    loop {
        let return_type = step(settings[0], first_input, amp_0_code);
        let (next_input, amp_0_code_new) = get_output(return_type);
        amp_0_code = amp_0_code_new;
        let return_type = step(settings[1], next_input, amp_1_code);
        let (next_input, amp_1_code_new) = get_output(return_type);
        amp_1_code = amp_1_code_new;

        let return_type = step(settings[2], next_input, amp_2_code);
        let (next_input, amp_2_code_new) = get_output(return_type);
        amp_2_code = amp_2_code_new;

        let return_type = step(settings[3], next_input, amp_3_code);
        let (next_input, amp_3_code_new) = get_output(return_type);
        amp_3_code = amp_3_code_new;

        let return_type= step(settings[4], next_input, amp_4_code);

        if let IntcodeReturnType::Interrupted(state) = return_type {
            amp_4_code = state.code;

            first_input = *state.output.first().unwrap();
        } else if let IntcodeReturnType::Finished(state) = return_type {
            return *state.output.first().unwrap();
        } else {
            panic!("not expected {:?}", return_type)
        }
    }

}

fn step(start_input: i64, second_input: i64, code: Vec<i64>) -> IntcodeReturnType {
    let intcode = run_instruction_set_with_input(code.clone(), start_input);
    return intcode.resume_with_input(second_input);
}

fn get_output(return_type: IntcodeReturnType) -> (i64, Memory) {
    if let IntcodeReturnType::Interrupted(state) = return_type {
        return (*state.output.first().unwrap(), state.code);
    }else if let IntcodeReturnType::Finished(state) = return_type {
        return (*state.output.first().unwrap(), state.code);
    } else {
        panic!("not expected {:?}", return_type)
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

    mod multi_run {
    use super::*;
        #[test]
        fn test_run_settings_1() {
            let strength = run_settings_until_halt(
                [9,8,7,6,5],
                &vec![
                    3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
                ],
            );

            assert_eq!(strength, 139629729);
        }

        #[test]
        fn test_run_settings_2() {
            let strength = run_settings_until_halt(
                [9,7,8,5,6],
                &vec![
                    3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                      -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                      53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
                ],
            );

            assert_eq!(strength, 18216);
        }

    }
}
