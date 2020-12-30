use std::{error::Error, str::FromStr, collections::HashSet};

use crate::intputer::{FromIntputerOutput, Intputer, UnexpectedOutputError};

pub fn day17_main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut intputer = Intputer::from_str(input)?;
    intputer.run()?;
    let output = intputer.get_entire_output::<char>()?.iter().collect::<String>();
    println!("{}", output);

    let mut scaffolding_places: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in output.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                scaffolding_places.insert((x as i32 ,y as i32));
            }
        }
    }

    let mut total_alignment = 0;
    for (x, y) in &scaffolding_places {
        if scaffolding_places.contains(&(x+1, *y)) && scaffolding_places.contains(&(x-1, *y))
            && scaffolding_places.contains(&(*x, y+1)) && scaffolding_places.contains(&(*x, y-1)) {
            let parameter = x*y;
            total_alignment = total_alignment + (parameter as i32);
        }
    }
    println!("The sum is {}", total_alignment);
    Ok(())
}

impl FromIntputerOutput for char {
    fn from_intputer_output(output: i64) -> Result<Self, crate::intputer::UnexpectedOutputError> {
        match output {
            0..=128 => Ok((output as u8) as char),
            val => Err(UnexpectedOutputError{output: val, message: "out of range".to_string()}),
        }
    }
}