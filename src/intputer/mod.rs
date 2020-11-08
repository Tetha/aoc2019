use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

mod addressing;

pub fn day2_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut testputer = Intputer::from_str("1,9,10,3,2,3,11,0,99,30,40,50")?;
    testputer.run()?;
    println!("{}", testputer);
    Ok(())
}

pub fn day6_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut testputer = Intputer::from_str("3,225,1,225,6,6,1100,1,238,225,104,0,1102,79,14,225,1101,17,42,225,2,74,69,224,1001,224,-5733,224,4,224,1002,223,8,223,101,4,224,224,1,223,224,223,1002,191,83,224,1001,224,-2407,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,18,64,225,1102,63,22,225,1101,31,91,225,1001,65,26,224,101,-44,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,101,78,13,224,101,-157,224,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,102,87,187,224,101,-4698,224,224,4,224,102,8,223,223,1001,224,4,224,1,223,224,223,1102,79,85,224,101,-6715,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1101,43,46,224,101,-89,224,224,4,224,1002,223,8,223,101,1,224,224,1,223,224,223,1101,54,12,225,1102,29,54,225,1,17,217,224,101,-37,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,20,53,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,226,226,224,1002,223,2,223,1006,224,329,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,344,101,1,223,223,7,677,226,224,102,2,223,223,1006,224,359,101,1,223,223,108,226,226,224,1002,223,2,223,1005,224,374,101,1,223,223,8,226,677,224,1002,223,2,223,1006,224,389,101,1,223,223,1108,226,226,224,102,2,223,223,1006,224,404,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,419,101,1,223,223,8,677,677,224,1002,223,2,223,1005,224,434,1001,223,1,223,1008,226,226,224,102,2,223,223,1005,224,449,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,464,101,1,223,223,1107,677,677,224,102,2,223,223,1006,224,479,101,1,223,223,107,677,677,224,1002,223,2,223,1005,224,494,1001,223,1,223,1107,226,677,224,1002,223,2,223,1005,224,509,101,1,223,223,1108,226,677,224,102,2,223,223,1006,224,524,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,539,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,554,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,569,1001,223,1,223,1008,677,677,224,102,2,223,223,1006,224,584,101,1,223,223,107,226,677,224,102,2,223,223,1005,224,599,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,614,101,1,223,223,1007,226,226,224,1002,223,2,223,1005,224,629,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,644,101,1,223,223,108,226,677,224,102,2,223,223,1006,224,659,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226")?;
    testputer.input.push(1);
    testputer.run()?;
    println!("{:?}", testputer.output);
    Ok(())
}

#[derive(Debug)]
struct Intputer {
    input: Vec<i32>,
    output: Vec<i32>,
    terminated: bool,
    position: i32,
    program : Vec<i32>,
}

#[derive(Debug, Clone)]
struct UnknownOpcode;

impl std::error::Error for UnknownOpcode {}
impl fmt::Display for UnknownOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown opcode encountered")
    }
}

impl Intputer {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", self.program);
        while !self.terminated {
            println!("{:?}", self);
            self.execute()
        }
        return Ok(())
    }
    fn execute(&mut self) {
        let current_instruction = self.program[self.position as usize];
        println!("{:?}", addressing::Instruction::from(current_instruction));
        match addressing::Instruction::from(current_instruction) {
            addressing::Instruction::Add(source1,  source2) => {
                let source1 = self.get_parameter(source1, 1);
                let source2 = self.get_parameter(source2, 2);
                let dest_pos = self.program[(self.position+3) as usize];
                self.program[dest_pos as usize] = source1 + source2;
                self.position = self.position + 4;
            }

            addressing::Instruction::Multiply(source1, source2) => {
                let source1 = self.get_parameter(source1, 1);
                let source2 = self.get_parameter(source2, 2);
                let dest_pos = self.program[(self.position+3) as usize];
                self.program[dest_pos as usize] = source1 * source2;
                self.position = self.position + 4;
            }

            addressing::Instruction::Input => {
                let input_value = self.input.remove(0);
                let dest_pos = self.program[(self.position+1) as usize];
                self.program[dest_pos as usize] = input_value;

                self.position = self.position + 2;
            }

            addressing::Instruction::Output(source) => {
                let source1 = self.get_parameter(source, 1);
                self.output.push(source1);
                self.position = self.position + 2;
            }

            addressing::Instruction::End => {
                self.terminated = true
            }

            addressing::Instruction::JumpIfTrue(source, dest) => {
                let source = self.get_parameter(source, 1);
                let dest = self.get_parameter(dest, 2);
                if source != 0 {
                    self.position = dest;
                } else {
                    self.position = self.position + 3;
                }
            }

            addressing::Instruction::Error => {
                println!("{:?}", self);
                panic!("Error instruction encountered")
            }
        }
    }

    fn get_parameter(&self, address_mode: addressing::AddressingMode, offset: i32) -> i32 {
        addressing::get_value(address_mode, self.program[(self.position+offset) as usize], &self.program)
    }
}

impl fmt::Display for Intputer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.program.iter().map(|i| { i.to_string() + ", " }).collect::<String>())
    }
}
impl FromStr for Intputer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unparsed_program: Vec<&str> = s.split(",").collect();
        let opcodes: Result<Vec<i32>, _> = unparsed_program.iter().map(|s| { s.parse::<i32>() }).collect();

        match opcodes {
            Ok(program) => Ok(Intputer{ 
                program: program,
                input: vec![],
                output: vec![],
                position: 0,
                terminated: false,
            }),
            Err(e) => Err(e)
        }
        
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*****
     * Opcode 1 - Add
     */
    #[test]
    fn test_add_immediate_position() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![1001, 4, 10, 5, 5, 0],
        };

        subject.execute();

        assert_eq!(subject.program, vec![1001, 4, 10, 5, 5, 15]);
        assert_eq!(subject.position, 4);
    }

    /*****
     * Opcode 2 - Multiply
     */
    #[test]
    fn test_mult_position_immediate() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![102, 9, 4, 5, 11, 0],
        };

        subject.execute();

        assert_eq!(subject.program, vec![102, 9, 4, 5, 11, 99]);
        assert_eq!(subject.position, 4);
    }

    /****
     * Opcode 3 - Input
     */
    #[test]
    fn test_input() {
        let mut subject = Intputer{
            input: vec![23],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![3, 2, 0],
        };

        subject.execute();

        assert_eq!(subject.position, 2);
        assert_eq!(subject.input, vec![]);
        assert_eq!(subject.program, vec![3, 2, 23]);
    }

    /*****
     * Opcode 4 - Output
     */
    #[test]
    fn test_output_immediate() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![104, 42],
        };

        subject.execute();

        assert_eq!(subject.position, 2);
        assert_eq!(subject.output, vec![42]);
        assert_eq!(subject.program, vec![104, 42]);
    }

    #[test]
    fn test_output_position() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![4, 2, 42],
        };

        subject.execute();

        assert_eq!(subject.position, 2);
        assert_eq!(subject.output, vec![42]);
        assert_eq!(subject.program, vec![4, 2, 42]);
    }


    /*****
     * Opcode 5 - jump if true
     */
    #[test]
    fn test_jump_if_true_position_immmediate_jumps() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![1005, 3, 42, 1],
        };
        
        subject.execute();

        assert_eq!(subject.position, 42);
    }

    #[test]
    fn test_jump_if_true_position_immmediate_noop() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![1005, 3, 42, 0],
        };
        
        subject.execute();

        assert_eq!(subject.position, 3);
    }
    /*****
     * Opcode 99 - Termination
     */
    #[test]
    fn test_termination() {
        let mut subject = Intputer{
            input: vec![],
            output: vec![],
            terminated: false,
            position: 0,
            program: vec![99],
        };

        subject.execute();

        assert_eq!(subject.terminated, true)
    }
}