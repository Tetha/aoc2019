
use std::{collections::HashMap, error::Error, str::FromStr, collections::VecDeque};

use crate::intputer::{FromIntputerOutput, Intputer, ToIntputerInput, UnexpectedOutputError};

#[derive(Debug, Clone, Copy)]
enum  MovementCommand {
    North,
    South,
    East,
    West,
}

impl ToIntputerInput for &MovementCommand {
    fn to_intputer_input(&self) -> i64 {
        match self {
            MovementCommand::North => 1,
            MovementCommand::South => 2,
            MovementCommand::West => 3,
            MovementCommand::East => 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RobotStatus {
    Wall,
    Stepped,
    NextToOxygen
}

impl FromIntputerOutput for RobotStatus {
    fn from_intputer_output(output: i64) -> Result<Self, UnexpectedOutputError> {
        match output {
            0 => Ok(RobotStatus::Wall),
            1 => Ok(RobotStatus::Stepped),
            2 => Ok(RobotStatus::NextToOxygen),
            _ => Err(UnexpectedOutputError{output: output, message: "cannot convert".to_string()})
        }
    }
}

fn convert_path_to_coordinates(path: &Vec<MovementCommand>) -> (i64, i64) {
    let x = path.iter().map(|p| match p {
        MovementCommand::North | MovementCommand::South => 0,
        MovementCommand::East => -1,
        MovementCommand::West => 1,
    }).sum();
    let y = path.iter().map(|p| match p {
        MovementCommand::East | MovementCommand::West => 0,
        MovementCommand::North => 1,
        MovementCommand::South => -1,
    }).sum();

    (x, y)
}

pub fn day_15_part1_main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut path = Vec::new();
    let mut tiles = HashMap::new();
    let reference_robot_program = Intputer::from_str(input)?;

    path.push(MovementCommand::North);
    visit(&mut path, &mut tiles, &reference_robot_program)?;
    path.pop();

    path.push(MovementCommand::East);
    visit(&mut path, &mut tiles, &reference_robot_program)?;
    path.pop();

    path.push(MovementCommand::West);
    visit(&mut path, &mut tiles, &reference_robot_program)?;
    path.pop();

    path.push(MovementCommand::South);
    visit(&mut path, &mut tiles, &reference_robot_program)?;
    path.pop();

    draw_robot_output(&tiles);

    println!("Shortest path is {}", find_shortest_path(&tiles));
    Ok(())
}

fn visit(path: &mut Vec<MovementCommand>, tiles: &mut HashMap<(i64,i64), RobotStatus>, reference_robot_program: &Intputer) -> Result<(), Box<dyn Error>> {
    let current_coords = convert_path_to_coordinates(path);
    if tiles.contains_key(&current_coords) {
        // field already explored
        return Ok(());
    }

    let mut local_robot = reference_robot_program.clone();
    local_robot.add_many_inputs(path.iter());
    let mut output = None;
    for _ in 0..path.len() {
        output = local_robot.run_until_new_output_values::<RobotStatus>()?;
    }
    let state = output.unwrap();

    tiles.insert(current_coords, state.clone());
    //println!("Path length: {}, tile count: {}", path.len(), tiles.len());

    match state {
        RobotStatus::Wall => Ok(()), // cannot continue here
        RobotStatus::NextToOxygen => Ok(()), // dont need to continue here (probably?)
        RobotStatus::Stepped => {
            path.push(MovementCommand::North);
            visit(path, tiles, reference_robot_program)?;
            path.pop();

            path.push(MovementCommand::East);
            visit(path, tiles, reference_robot_program)?;
            path.pop();

            path.push(MovementCommand::West);
            visit(path, tiles, reference_robot_program)?;
            path.pop();

            path.push(MovementCommand::South);
            visit(path, tiles, reference_robot_program)?;
            path.pop();

            return Ok(())
        }
    }
}

fn find_shortest_path(tiles: &HashMap<(i64, i64), RobotStatus>) -> i64 {
    let mut parent: HashMap<(i64,i64), (i64,i64)> = HashMap::new();
    let mut queue: VecDeque<(i64,i64)> = VecDeque::new();
    parent.insert((0,0), (0,0));
    queue.push_front((0,0));
    let mut goal: Option<(i64,i64)> = None;
    while !queue.is_empty() && goal.is_none() {
        let v = queue.pop_back().unwrap(); // None is impossible due to the condition
        let state = tiles[&v];

        match state {
            RobotStatus::Wall => continue,
            RobotStatus::NextToOxygen => goal = Some(v),
            RobotStatus::Stepped => {
                let (x, y) = v;
                for next in vec![(x+1, y), (x-1, y), (x, y-1), (x, y+1)] {
                    if !parent.contains_key(&next) {
                        parent.insert(next, v);
                        queue.push_front(next);
                    }
                }
            }
        }
    }

    let mut path_length = 0;
    let mut current = goal.unwrap();

    while current != (0,0) {
        path_length = path_length + 1;
        current = parent[&current];
    }

    return path_length;
}

fn draw_robot_output(tiles: &HashMap<(i64,i64), RobotStatus>) {
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
            if tiles.contains_key(&(x,y)) {
                match tiles[&(x,y)] {
                    RobotStatus::Stepped => lines.push('_'),
                    RobotStatus::Wall => lines.push('#'),
                    RobotStatus::NextToOxygen => lines.push('*'),
                }
            } else {
                lines.push(' ');
            }
        }
        lines.push('\n');
    }

    println!("{}", lines.iter().collect::<String>());
}