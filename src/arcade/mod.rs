
use std::str::FromStr;
use itertools::Itertools;

use super::intputer::Intputer;

pub fn day13_part1_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("program");
    let mut puter = Intputer::from_str(input)?;
    puter.run()?;

    let mut blocks = 0;
    for chunk in &puter.output.into_iter().chunks(3) {
        let chunk = chunk.collect::<Vec<i64>>();
        if chunk[2] == 2 {
            blocks = blocks + 1;
        }
    }

    println!("There are {} blocks", blocks);
    Ok(())
}