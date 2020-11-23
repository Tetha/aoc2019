
use itertools;


pub fn day16_part2_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let digits: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let many_digits: Vec<i32> = digits.iter().cycle().take(10000 * digits.len()).map(|x| *x).collect();
    let output = iterate_often(&many_digits, 100);
    let message_offset = 59709511;
    for i in message_offset..=message_offset+8 {
        print!("{}", output[i]);
    }
    //println!("Result is {}", output.iter().map(|d| d.to_string()).collect::<String>());
    Ok(())
}
pub fn day16_part1_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let digits: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let output = iterate_often(&digits, 100);
    println!("Result is {}", output.iter().map(|d| d.to_string()).collect::<String>());
    Ok(())
}
fn iterate_often(input: &Vec<i32>, phases: i32) -> Vec<i32> {
    let mut current = iterate(input);

    for _ in 1..phases {
        current = iterate(&current);
    }

    current
}

fn iterate(input: &Vec<i32>) -> Vec<i32> {
    (1..=input.len()).map(|i| iterate_position(input, i as i32)).collect::<Vec<i32>>()
}
fn iterate_position(input: &Vec<i32>, pos: i32) -> i32 {
    /*
    println!("{}", input.iter()
                        .zip(create_multiplication_pattern(pos))
                        .map(|(a,b)| a*b)
                        .sum::<i32>());*/
    input.iter()
         .zip(create_multiplication_pattern(pos))
         .map(|(a,b)| *a*b)
         .sum::<i32>().abs() % 10
}

fn create_multiplication_pattern(pos: i32) -> impl Iterator<Item=i32> {
    itertools::repeat_n(0, pos as usize)
            .chain(itertools::repeat_n(1, pos as usize))
            .chain(itertools::repeat_n(0, pos as usize))
            .chain(itertools::repeat_n(-1, pos as usize))
            .cycle()
            .skip(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_pattern_pos_2() {
        let sample: Vec<i32> = create_multiplication_pattern(2).take(15).collect();
        assert_eq!(sample, vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_iteration_one() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(iterate_position(&input, 1), 4);
        assert_eq!(iterate_position(&input, 2), 8);
        assert_eq!(iterate_position(&input, 3), 2);
        assert_eq!(iterate_position(&input, 4), 2);
        assert_eq!(iterate_position(&input, 5), 6);
        assert_eq!(iterate_position(&input, 6), 1);
        assert_eq!(iterate_position(&input, 7), 5);
        assert_eq!(iterate_position(&input, 8), 8);
    }

    #[test]
    fn test_position_iteration() {
        let mut value = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // phase 1
        value = iterate(&value);
        assert_eq!(value, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        // phase 2
        value = iterate(&value);
        assert_eq!(value, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        // phase 3
        value = iterate(&value);
        assert_eq!(value, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        // phase 4
        value = iterate(&value);
        assert_eq!(value, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_many_iterations() {
        let input = vec![8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5, 5, 9, 5];
        let output = iterate_often(&input, 100);
        assert_eq!(output[0], 2);
        assert_eq!(output[1], 4);
        assert_eq!(output[2], 1);
        assert_eq!(output[3], 7);
        assert_eq!(output[4], 6);
        assert_eq!(output[5], 1);
        assert_eq!(output[6], 7);
        assert_eq!(output[7], 6);
    }
}