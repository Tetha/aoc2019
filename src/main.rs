use std::env;
use std::fmt;
use std::error;

mod day5_passwords;
mod intputer;
mod orbits;
mod accumulator;
mod space_image_password;
mod asteroids;
mod hull_painter;
mod jupiter;
mod arcade;

#[derive(Debug, Clone)]
struct UsageError {
    cause: String
}

impl UsageError {
    fn new(cause: String) -> UsageError {
        UsageError{ cause: cause }
    }
} 
impl error::Error for UsageError {}
impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing Argument")
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::new(UsageError::new(String::from("missing day arg"))))
    }

    let command = &args[1];
    match command as &str {
        "day2" => intputer::day2_main(),
        "day4" => Ok(day5_passwords::day4_main()),
        "day5" => intputer::day5_main(1),
        "day5_part2" => intputer::day5_main(5),
        "day6" => orbits::day6_main(),
        "day6_part2" => orbits::day6_part2_main(),
        "day7" => accumulator::day7_main(),
        "day8" => space_image_password::day8_part1_main(),
        "day8_part2" => space_image_password::day8_part2_main(),
        "day9" => intputer::day9_main(),
        "day9_part2" => intputer::day9_part2_main(),
        "day10" => asteroids::day10_main(),
        "day11" => hull_painter::day11_main(),
        "day11_part2" => hull_painter::day11_part2_main(),
        "day12_test" => jupiter::day12_test_main(),
        "day12" => jupiter::day12_part1_main(),
        "day13" => arcade::day13_part1_main(),
        arg => Err(Box::new(UsageError::new(format!("Unknown argument {}", arg))))

    }
}