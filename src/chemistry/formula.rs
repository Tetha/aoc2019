use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use super::Element;

#[derive(Debug)]
pub struct ReactionFormula {
    pub output: Element,
    pub amount: i32,

    pub inputs: HashMap<Element, i32>,
}

#[derive(Debug)]
pub struct ReactionParseError { 
    message: String,
}
impl ReactionParseError {
    fn new(message: String) -> ReactionParseError {
        return ReactionParseError{message};
    }
}
impl Display for ReactionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "unable to parse reaction: {}", self.message)
    }
}

impl From<ParseIntError> for ReactionParseError {
    fn from(err: ParseIntError) -> ReactionParseError {
        ReactionParseError::new(format!("unable to parse amount: {}", err))
    }
}
impl Error for ReactionParseError {}

impl FromStr for ReactionFormula {
    type Err = ReactionParseError;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let parts = input.split("=>").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(ReactionParseError::new(format!("expected input to contain '=>': {}", input)));
        }
        let (amount, output) = split_amount_element(parts[1])?;

        let mut inputs: HashMap<String, i32> = HashMap::new();
        for input in parts[0].trim().split(",") {
            let (input_amount, input_element) = split_amount_element(input)?;
            inputs.insert(input_element.to_string(), input_amount);
        }

        Ok(ReactionFormula{
            output: output.to_string(),
            amount: amount,
            inputs: inputs,
        })
    }
}

fn split_amount_element(input: &str) -> Result<(i32, &str), ReactionParseError> {
    let chunks = input.trim().split(" ").collect::<Vec<&str>>();
    if chunks.len() != 2 {
        return Err(ReactionParseError::new(format!("expected space in chunk {}", input)));
    }
    let amount = chunks[0].trim().parse::<i32>()?;
    let element = chunks[1].trim();
    return Ok((amount, element));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let input = "23 ORE => 42 FUEL";
        let subject = ReactionFormula::from_str(input).unwrap();
        assert_eq!("FUEL", subject.output);
        assert_eq!(42, subject.amount);
        assert_eq!(23, subject.inputs["ORE"]);
    }

    #[test]
    fn test_harder_parse() {
        let input = "4 A, 23 B, 49 ORE => 42 FUEL";
        let subject = ReactionFormula::from_str(input).unwrap();
        assert_eq!("FUEL", subject.output);
        assert_eq!(42, subject.amount);
        assert_eq!(49, subject.inputs["ORE"]);
        assert_eq!(4, subject.inputs["A"]);
        assert_eq!(23, subject.inputs["B"]);
    }
}