fn main() {
    let low = 246515;
    let high = 739105;

    let numbers = (low..high).filter(|&nr| test_number(nr)).count();
    println!("Unique passwords: {}", numbers);

    let numbers = (low..high).filter(|&nr| test_number_more_infos(nr)).count();
    println!("Unique passwords: {}", numbers);
}

fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(6);
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn is_not_decreasing(num: &Vec<i32>) -> bool {
    num.windows(2).all(|a| a[0] <= a[1])
}

fn has_double_digit(num: &Vec<i32>) -> bool {
    num.windows(2).any(|a| a[0] == a[1])
}

fn has_isolated_double_digit(num: &Vec<i32>) -> bool {
    let mut was_double = false;
    let mut legal_double = false;
    let mut current_candidate = num[0];
    for &digit in num.iter().skip(1) {
        if digit != current_candidate {
            if legal_double {
                return true;
            }
            was_double = false;
            current_candidate = digit;
        } else if was_double {
            legal_double = false
        } else {
            legal_double = true;
            was_double = true;
        }
    }
    legal_double
}

fn test_number(num: i32) -> bool {
    let num_as_vec = number_to_vec(num);
    is_not_decreasing(&num_as_vec) && has_double_digit(&num_as_vec)
}

fn test_number_more_infos(num: i32) -> bool {
    let num_as_vec = number_to_vec(num);
    is_not_decreasing(&num_as_vec) && has_isolated_double_digit(&num_as_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_decreasing() {
        assert!(is_not_decreasing(&vec![1, 1, 1, 1]));
        assert!(is_not_decreasing(&vec![1, 1, 2, 3]));
        assert!(!is_not_decreasing(&vec![1, 1, 2, 3, 0]));
    }

    #[test]
    fn test_has_double_digit() {
        assert!(has_double_digit(&vec![1, 1, 0, 1]));
        assert!(has_double_digit(&vec![1, 1, 2, 3]));
        assert!(!has_double_digit(&vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_has_lone_double_digit() {
        assert!(has_isolated_double_digit(&vec![1, 1, 0, 1]));
        assert!(has_isolated_double_digit(&vec![1, 2, 3, 3]));
        assert!(!has_isolated_double_digit(&vec![1, 1, 1, 2]));
        assert!(!has_isolated_double_digit(&vec![2, 1, 1, 1]));
        assert!(!has_isolated_double_digit(&vec![2, 1, 1, 1, 2]));
        assert!(has_isolated_double_digit(&vec![2, 2, 1, 1, 1, 2]));
    }

    #[test]
    fn test_test_number() {
        assert!(test_number(111111));
        assert!(!test_number(223450));
        assert!(!test_number(123789));
    }
}
