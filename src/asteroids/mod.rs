use std::str::FromStr;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
struct Asteroid {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct AsteroidField {
    astroids: Vec<Asteroid>,
}

#[derive(Debug)]
enum AsteroidFieldErrorKind {
    InvalidCharacter,
}

#[derive(Debug)]
struct AsteroidFieldError{ 
    kind: AsteroidFieldErrorKind,
    message: String,
}

impl Display for AsteroidFieldError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        fmt.write_fmt(format_args!("astroid field error: {}", self.message))
    }
}
impl Error for AsteroidFieldError {}

pub fn day10_main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input");
    let field = AsteroidField::from_str(input)?;
    let positions = field.astroids.iter().max_by_key(|a| field.remove_blocked_astroids(field.place_possible_observation_station(a.x, a.y)).len());
    println!("Possible positions: {:?}", positions);
    if let Some(Asteroid{x, y}) = positions {
        println!("It sees {} astroids", field.remove_blocked_astroids(field.place_possible_observation_station(*x, *y)).len());
    }
    Ok(())
}
impl FromStr for AsteroidField {
    type Err = AsteroidFieldError;

    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut result = AsteroidField{astroids: Vec::new()};
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                match value {
                    '.' => continue,
                    '#' => result.astroids.push(Asteroid{x: x as i32, y: y as i32}),
                    _ => return Err(AsteroidFieldError{
                        kind: AsteroidFieldErrorKind::InvalidCharacter, 
                        message: format!("invalid character: <{}>", value)
                    })
                }
            }
        }
        Ok(result)
    }
}

impl AsteroidField {
    fn place_possible_observation_station(&self, x: i32, y: i32) -> Vec<LineToAstroid> {
        self.astroids.iter()
                     .filter(|a| !(a.x == x && a.y == y))
                     .map(|a| LineToAstroid{ dx: a.x - x, dy: a.y - y})
                     .collect()
    }

    fn remove_blocked_astroids(&self, lines: Vec<LineToAstroid>) -> Vec<LineToAstroid> {
        //println!("-------");
        lines.iter()
             .filter(|a| !lines.iter().any(|a2| {
                 let result = a2.blocks(a);
                 //println!("{:?}.blocks({:?}) -> {}", a2, a, result);
                 return result;
              }))
             .map(|a| *a)
             .collect()
    }

    fn find_best_station(&self) -> Option<&Asteroid> {
        self.astroids.iter().max_by_key(|a| {
            let unblocked_astroids = self.remove_blocked_astroids(self.place_possible_observation_station(a.x, a.y));
            let result = unblocked_astroids.len();
            println!("Asteroid {:?} has result {:?} - {:?}", a, result, unblocked_astroids);
            return result;
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct LineToAstroid {
    dx: i32,
    dy: i32,
}
impl LineToAstroid {
    fn blocks(&self, other: &LineToAstroid) -> bool {
        if self.dy == 0 && other.dy == 0 {
            if self.dx < 0 {
                return self.dx > other.dx;
            } else {
                return self.dx < other.dx;
            }
        } else {
            let t = self.dy as f32 / other.dy as f32;
            if t < 0.0 {
                return false;
            }

            if self.dx as f32 / t != other.dx as f32 {
                return false;
            }
            return self.dx * self.dx + self.dy * self.dy  < other.dx * other.dx + other.dy * other.dy;

        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_simple_parse() {
        let input = ".#
#.";
        let subject = AsteroidField::from_str(input).unwrap();
        assert_eq!(subject.astroids[0], Asteroid{ x: 1, y: 0});
        assert_eq!(subject.astroids[1], Asteroid{ x: 0, y: 1});
    }

    #[test]
    fn test_simple_block() {
        let line1 = LineToAstroid{dx: 1, dy: 3};
        let line2 = LineToAstroid{dx: 2, dy: 6};
        assert_eq!(true, line1.blocks(&line2));
        assert_eq!(false, line2.blocks(&line1));
    }

    #[test]
    fn test_self_block() {
        let line1 = LineToAstroid{dx: 1, dy: 3};
        assert_eq!(false, line1.blocks(&line1));
    }

    #[test]
    fn test_other_quadrant_block() {
        let line1 = LineToAstroid{dx: 1, dy: 1};
        let line2 = LineToAstroid{dx: -1, dy: -1};
        assert_eq!(false, line1.blocks(&line2));
        assert_eq!(false, line2.blocks(&line1));
    }

    #[test]
    fn test_dy_zero() {
        let line1 = LineToAstroid{dx: 1, dy: 0};
        let line2 = LineToAstroid{dx: 2, dy: 0};
        assert_eq!(true, line1.blocks(&line2));
        assert_eq!(false, line2.blocks(&line1));
    }

    #[test]
    fn test_dx_zero() {
        let line1 = LineToAstroid{dx: 3, dy: 4};
        let line2 = LineToAstroid{dx: 0, dy: 2};
        assert_eq!(false, line1.blocks(&line2));
        assert_eq!(false, line2.blocks(&line1));
    }

    #[test]
    fn test_example_1() {
        let input = ".#..#
.....
#####
....#
...##";

        let subject = AsteroidField::from_str(input).unwrap();
        let best_place = subject.find_best_station().unwrap();

        assert_eq!(*best_place, Asteroid{x: 3, y: 4});
        let astroid_count = subject.remove_blocked_astroids(subject.place_possible_observation_station(best_place.x, best_place.y)).len();
        assert_eq!(astroid_count, 8);
    }
}