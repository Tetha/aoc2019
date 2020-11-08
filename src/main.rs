use std::env;
use std::fmt;
use std::error;

mod day5_passwords;
mod intputer;

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
        "day5" => Ok(day5_passwords::day5_main()),
        "day6" => intputer::day6_main(),
        arg => Err(Box::new(UsageError::new(format!("Unknown argument {}", arg))))

    }
}