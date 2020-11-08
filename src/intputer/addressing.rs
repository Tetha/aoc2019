#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Immediate,
    Position,
    Error,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(AddressingMode, AddressingMode),
    Multiply(AddressingMode, AddressingMode),
    Input,
    Output(AddressingMode),
    JumpIfTrue(AddressingMode, AddressingMode),
    End,
    Error
}

impl From<i32> for AddressingMode {
    fn from(mode: i32) -> AddressingMode {
        match mode {
            0 => AddressingMode::Position,
            1 => AddressingMode::Immediate,
            _ => AddressingMode::Error,
        }
    }
}
impl From<i32> for Instruction {
    fn from(opcode: i32) -> Instruction {
        let instruction_code = opcode % 100;
        let parameter_1_mode = (opcode / 100) % 10;
        let parameter_2_mode = (opcode / 1000) % 10;
        //let parameter_3_mode = (opcode / 10000) % 10;

        match instruction_code {
            1 => Instruction::Add(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
            ),
            2 => Instruction::Multiply(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
            ),
            3 => Instruction::Input,
            4 => Instruction::Output(
                AddressingMode::from(parameter_1_mode),
            ),
            5 => Instruction::JumpIfTrue(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
            ),
            99 => Instruction::End,
            _ => Instruction::Error
        }
    }
}


pub fn get_value(mode: AddressingMode, parameter: i32, program: &Vec<i32>) -> i32 {
    match mode {
        AddressingMode::Immediate => parameter,
        AddressingMode::Position => program[parameter as usize],
        AddressingMode::Error => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump_if_true_decoding() {
        let jump_with_position_parameter = 5;
        assert_eq!(Instruction::from(
            jump_with_position_parameter),
            Instruction::JumpIfTrue(AddressingMode::Position, AddressingMode::Position)
        );

        let jump_with_immediate_parameter = 105;
        assert_eq!(
            Instruction::from(jump_with_immediate_parameter),
            Instruction::JumpIfTrue(AddressingMode::Immediate, AddressingMode::Position)
        );

        let jump_with_immediate_parameter = 1005;
        assert_eq!(
            Instruction::from(jump_with_immediate_parameter),
            Instruction::JumpIfTrue(AddressingMode::Position, AddressingMode::Immediate)
        );
    }
}
