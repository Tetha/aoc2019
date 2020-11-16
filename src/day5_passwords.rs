pub fn day4_main() {
    println!("Hello, world!");
    let start = 171309;
    let end = 643603;

    let mut passwords = 0;

    for i in start..=end {
        let digits = digits(i);
        if check_sequence(digits) {
            passwords += 1;
        }
    }

    println!("{} passwords found", passwords);
}

fn check_sequence(digits: Vec<i32>) -> bool {
    let mut duplicate_found = false;
    let mut decrease_found = false;

    for i in 0..digits.len()-1 {
        let digit = digits[i];
        let next_digit = digits[i+1];

        if digit == next_digit {
            duplicate_found = true;
        }

        if digit > next_digit {
            decrease_found = true;
        }
    }
    duplicate_found && !decrease_found
}

fn digits(number: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();

    let mut current_rest = number;
    while current_rest > 0 {
        result.push(current_rest % 10);
        current_rest = current_rest / 10;
    }
    result.iter().rev().cloned().collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(123), vec![1,2,3]);
    }

    #[test]
    fn test_check_sequence() {
        assert_eq!(check_sequence(vec![1, 1, 1, 1, 1, 1]), true);
        assert_eq!(check_sequence(vec![2, 2, 3, 4, 5, 0]), false);
        assert_eq!(check_sequence(vec![1, 2, 3, 7, 8, 9]), false);
    }
}