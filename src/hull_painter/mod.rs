
use defaultmap;

use std::convert::TryFrom;
use std::fmt::Display;
use std::error::Error;
use std::str::FromStr;

use super::intputer;

mod field_color;

pub fn day11_main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut brain = intputer::Intputer::from_str(input)?;
    //brain.debug = true;
    let mut robot = HullPainterRobot::new(brain);
    let paint = robot.run(field_color::FieldColor::Black);
    println!("{:?}", paint);
    println!("{:?}", paint.len());
    Ok(())
}
pub fn day11_part2_main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut brain = intputer::Intputer::from_str(input)?;
    //brain.debug = true;
    let mut robot = HullPainterRobot::new(brain);
    let paint = robot.run(field_color::FieldColor::White);
    println!("{:?}", paint);
    println!("{:?}", paint.len());
    draw_robot_output(&paint);
    Ok(())
}

fn draw_robot_output(tiles: &defaultmap::DefaultHashMap<(i32,i32), field_color::FieldColor>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in tiles.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    let mut lines = Vec::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match tiles[(x,y)] {
                field_color::FieldColor::Black => lines.push(' '),
                field_color::FieldColor::White => lines.push('#'),
            }
        }
        lines.push('\n');
    }

    println!("{}", lines.iter().collect::<String>());
}

enum Heading {
    Up, Down, Left, Right
}

enum Turn {
    Left, Right
}

#[derive(Debug)]
pub struct TurnError {source: i64}

impl Error for TurnError{}

impl Display for TurnError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        fmt.write_fmt(format_args!("unable to map number {}", self.source))
    }
}

impl TryFrom<i64> for Turn {
    type Error = TurnError;
    fn try_from(i: i64) -> std::result::Result<Self, <Self as std::convert::TryFrom<i64>>::Error> {
        match i {
            0 => Ok(Turn::Left),
            1 => Ok(Turn::Right),
            _ => Err(TurnError{source: i}),
        }
    }
}

struct HullPainterRobot {
    brain: intputer::Intputer,
    heading: Heading,

    x: i32,
    y: i32,
}

impl HullPainterRobot {
    fn new(brain: intputer::Intputer) -> HullPainterRobot {
        return HullPainterRobot{
            brain: brain,
            heading: Heading::Up,
            x: 0,
            y: 0,
        }
    }

    fn turn(&mut self, turn: Turn) {
        self.heading = match turn {
            Turn::Left => match self.heading {
                        Heading::Up => Heading::Left,
                        Heading::Down => Heading::Right,
                        Heading::Right => Heading::Up,
                        Heading::Left => Heading::Down,
            },
            Turn::Right => match self.heading {
                        Heading::Up => Heading::Right,
                        Heading::Down => Heading::Left,
                        Heading::Right => Heading::Down,
                        Heading::Left => Heading::Up,
            },

        }
    }
    fn move_forward(&mut self) {
        match self.heading {
            Heading::Up => self.y = self.y + 1,
            Heading::Down => self.y = self.y - 1,
            Heading::Left => self.x = self.x - 1,
            Heading::Right => self.x = self.x + 1,
        }
    }

    fn run(&mut self, start_color: field_color::FieldColor) -> defaultmap::DefaultHashMap<(i32, i32), field_color::FieldColor> {
        let mut colors = defaultmap::DefaultHashMap::new(field_color::FieldColor::Black);
        colors[(0,0)] = start_color;

        loop {
            if self.brain.debug {
                println!("Color: {:?} <- {}/{}", colors, self.x, self.y);
            }
            self.brain.input.push(colors.get((self.x, self.y)).to_intputer_input());
            match self.brain.run_until_new_output_values(2) {
                None => return colors,
                Some((paint_directive, direction_output)) => {
                    colors[(self.x, self.y)] = field_color::FieldColor::try_from(paint_directive).unwrap();
                    self.turn(Turn::try_from(direction_output).unwrap());
                    self.move_forward()
                }
            }
        }
    }
}