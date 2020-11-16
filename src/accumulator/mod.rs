
use super::intputer;
use std::str::FromStr;

use permutohedron::Heap;

pub fn day7_main() -> Result<(), Box<dyn std::error::Error>> {
    let accumulator_program = include_str!("input");
    let (phase_setting, output) = find_thruster_settings(accumulator_program);
    println!("Best Phase Setting is {:?} with output {:?}", phase_setting, output);
    Ok(())
}

fn find_thruster_settings(program: &str) -> (Option<Vec<i32>>, Option<i32>) {
    let mut possible_phases = vec![0,1,2,3,4];
    let possible_phases = Heap::new(&mut possible_phases);

    let mut best_setting_output = None;
    let mut best_phase_setting = None;
    for permutation in possible_phases {
        let current_output = run_accumulator_simulation(&permutation, program);
        match best_setting_output {
            None => {
                best_setting_output = Some(current_output);
                best_phase_setting = Some(permutation.clone());
            },
            Some(output) => {
                if current_output > output {
                    best_setting_output = Some(current_output);
                    best_phase_setting = Some(permutation.clone());
                }
            }
        }
    }
    return (best_phase_setting, best_setting_output)
}

fn run_accumulator_simulation(phase_setting: &Vec<i32>, program: &str) -> i32 {
    let accumulator_base_intputer = intputer::Intputer::from_str(program).unwrap();

    let mut current_output = 0;

    for phase in phase_setting {
        let mut accumulator = accumulator_base_intputer.clone();
        accumulator.input.push(*phase);
        accumulator.input.push(current_output);
        accumulator.run().unwrap();
        current_output = accumulator.output[0];
    }

    current_output
}

mod tests {
    use super::run_accumulator_simulation;

    #[test]
    fn test_example_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let phase_setting = vec![4,3,2,1,0];
        let actual_output = run_accumulator_simulation(&phase_setting, program);
        assert_eq!(43210, actual_output);
    }

    #[test]
    fn test_example_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let phase_setting = vec![0,1,2,3,4];
        let actual_output = run_accumulator_simulation(&phase_setting, program);
        assert_eq!(54321, actual_output);
    }

    #[test]
    fn test_example_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let phase_setting = vec![1,0,4,3,2];
        let actual_output = run_accumulator_simulation(&phase_setting, program);
        assert_eq!(65210, actual_output);
    }
}