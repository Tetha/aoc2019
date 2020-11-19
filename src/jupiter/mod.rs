
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

mod vector;
use vector::Vector;

pub fn day12_part1_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<x=5, y=-1, z=5>
<x=0, y=-14, z=2>
<x=16, y=4, z=0>
<x=18, y=1, z=16>";

    let mut moons = input.lines().map(|l| Moon::from_str(l)).collect::<Result<Vec<Moon>,MoonParseError>>()?;
    let mut step = 0;
    println!("Step #{}: ", step);
    for moon in &moons {
        println!("{}", moon);
    }
    for _ in 0..1000 {
        step = step+1;
        step_simulation(&mut moons)
    }
    println!("Step #{}: ", step);
    for moon in &moons {
        println!("{}", moon);
    }
    println!("The energy is {}", compute_energy(&moons));
    Ok(())
}

pub fn day12_test_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    let mut moons = input.lines().map(|l| Moon::from_str(l)).collect::<Result<Vec<Moon>,MoonParseError>>()?;
    let mut step = 0;
    println!("Step #{}: ", step);
    for moon in &moons {
        println!("{}", moon);
    }
    for _ in 0..10 {
        step = step+1;
        step_simulation(&mut moons)
    }
    println!("Step #{}: ", step);
    for moon in &moons {
        println!("{}", moon);
    }
    println!("The energy is {}", compute_energy(&moons));
    Ok(())
}

fn compute_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(|m| compute_energy_moon(m)).sum()
}

fn compute_energy_moon(moon: &Moon) -> i32 {
    let potential_energy =moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
    let kinetic_energy = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
    potential_energy * kinetic_energy 
}
fn step_simulation(moons: &mut Vec<Moon>) {
    apply_gravity(moons);
    apply_velocity(moons);
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            //println!("Applying pair moon[{}] and moon[{}] / {} - {}", i, j, moons[i], moons[j]);
            if moons[i].position.x != moons[j].position.x {
                if moons[i].position.x < moons[j].position.x {
                    moons[i].velocity.x = moons[i].velocity.x + 1;
                    moons[j].velocity.x = moons[j].velocity.x - 1;
                } else {
                    moons[i].velocity.x = moons[i].velocity.x - 1;
                    moons[j].velocity.x = moons[j].velocity.x + 1;
                }
            }
            if moons[i].position.y != moons[j].position.y {
                if moons[i].position.y < moons[j].position.y {
                    moons[i].velocity.y = moons[i].velocity.y + 1;
                    moons[j].velocity.y = moons[j].velocity.y - 1;
                } else {
                    moons[i].velocity.y = moons[i].velocity.y - 1;
                    moons[j].velocity.y = moons[j].velocity.y + 1;
                }
            }
            if moons[i].position.z != moons[j].position.z {
                if moons[i].position.z < moons[j].position.z {
                    moons[i].velocity.z = moons[i].velocity.z + 1;
                    moons[j].velocity.z = moons[j].velocity.z - 1;
                } else {
                    moons[i].velocity.z = moons[i].velocity.z - 1;
                    moons[j].velocity.z = moons[j].velocity.z + 1;
                }
            }
            //println!("Adjusted to moon[{}] and moon[{}] / {} - {}", i, j, moons[i], moons[j]);
        }    
    }
}

fn apply_velocity(moons: &mut Vec<Moon>) {
    for moon in moons.iter_mut() {
        moon.position.x = moon.position.x + moon.velocity.x;
        moon.position.y = moon.position.y + moon.velocity.y;
        moon.position.z = moon.position.z + moon.velocity.z;
    }
}
#[derive(Debug, Clone)]
struct Moon {
    velocity: Vector,
    position: Vector,
}

impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "(pos={}, vel={})", self.position, self.velocity)
    }
}

impl FromStr for Moon {
    type Err = MoonParseError;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Moon{
            position: Vector::from_str(input)?,
            velocity: Vector{x:0, y:0, z:0},
        })
    }
}

#[derive(Debug,Clone,PartialEq)]
struct MoonParseError{
    message: String
}

impl Display for MoonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "unable to parse moon: {}", self.message)
    }
}

impl Error for MoonParseError {}

impl From<vector::VectorParseError> for MoonParseError {
    fn from(vector_error: vector::VectorParseError) -> Self {
        MoonParseError{message: format!("unable to parse position {}", vector_error)}
    }
}