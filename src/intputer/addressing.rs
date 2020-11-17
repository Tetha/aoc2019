#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Immediate,
    Position,
    Relative,
    Error,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(AddressingMode, AddressingMode, AddressingMode),
    Multiply(AddressingMode, AddressingMode, AddressingMode),
    Input { destination: AddressingMode },
    Output(AddressingMode),
    JumpIfTrue(AddressingMode, AddressingMode),
    JumpIfFalse(AddressingMode, AddressingMode),

    CompareLessThan(AddressingMode, AddressingMode, AddressingMode),
    CompareEquals(AddressingMode, AddressingMode, AddressingMode),

    SetRelativeBase(AddressingMode),

    End,
    Error
}

impl From<i64> for AddressingMode {
    fn from(mode: i64) -> AddressingMode {
        match mode {
            0 => AddressingMode::Position,
            1 => AddressingMode::Immediate,
            2 => AddressingMode::Relative,
            _ => AddressingMode::Error,
        }
    }
}
impl From<i64> for Instruction {
    fn from(opcode: i64) -> Instruction {
        let instruction_code = opcode % 100;
        let parameter_1_mode = (opcode / 100) % 10;
        let parameter_2_mode = (opcode / 1000) % 10;
        let parameter_3_mode = (opcode / 10000) % 10;

        match instruction_code {
            1 => Instruction::Add(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
                AddressingMode::from(parameter_3_mode),
            ),
            2 => Instruction::Multiply(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
                AddressingMode::from(parameter_3_mode),
            ),
            3 => Instruction::Input {
                destination: AddressingMode::from(parameter_1_mode),
            },
            4 => Instruction::Output(
                AddressingMode::from(parameter_1_mode),
            ),
            5 => Instruction::JumpIfTrue(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
            ),
            6 => Instruction::JumpIfFalse(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
            ),
            7 => Instruction::CompareLessThan(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
                AddressingMode::from(parameter_3_mode),
            ),
            8 => Instruction::CompareEquals(
                AddressingMode::from(parameter_1_mode),
                AddressingMode::from(parameter_2_mode),
                AddressingMode::from(parameter_3_mode),
            ),
            9 => Instruction::SetRelativeBase(
                AddressingMode::from(parameter_1_mode),
            ),
            99 => Instruction::End,
            _ => Instruction::Error
        }
    }
}


pub fn get_value(mode: AddressingMode, parameter: i64, program: &Vec<i64>, relative_base: i64) -> i64 {
    match mode {
        AddressingMode::Immediate => parameter,
        AddressingMode::Position => program[parameter as usize],
        AddressingMode::Relative => program[(relative_base + parameter) as usize],
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

    #[test]
    fn test_jump_if_false_decoding() {
        assert_eq!(
            Instruction::from(6), 
            Instruction::JumpIfFalse(AddressingMode::Position, AddressingMode::Position));
        assert_eq!(
            Instruction::from(1106),
            Instruction::JumpIfFalse(AddressingMode::Immediate, AddressingMode::Immediate));
    }

    #[test]
    fn test_compare_less_than() {
        assert_eq!(
            Instruction::from(7),
            Instruction::CompareLessThan(AddressingMode::Position, AddressingMode::Position, AddressingMode::Position));
        assert_eq!(
            Instruction::from(107),
            Instruction::CompareLessThan(AddressingMode::Immediate, AddressingMode::Position, AddressingMode::Position));
        assert_eq!(
            Instruction::from(1007),
            Instruction::CompareLessThan(AddressingMode::Position, AddressingMode::Immediate, AddressingMode::Position));
    }

    #[test]
    fn test_compare_equals() {
        assert_eq!(
            Instruction::from(8),
            Instruction::CompareEquals(AddressingMode::Position, AddressingMode::Position, AddressingMode::Position));
        assert_eq!(
            Instruction::from(108),
            Instruction::CompareEquals(AddressingMode::Immediate, AddressingMode::Position, AddressingMode::Position));
        assert_eq!(
            Instruction::from(1008),
            Instruction::CompareEquals(AddressingMode::Position, AddressingMode::Immediate, AddressingMode::Position));
    }

    fn test_set_relative_base() {
        assert_eq!(
            Instruction::from(9),
            Instruction::SetRelativeBase(AddressingMode::Position));
        assert_eq!(
            Instruction::from(109),
            Instruction::SetRelativeBase(AddressingMode::Immediate));
        assert_eq!(
            Instruction::from(209),
            Instruction::SetRelativeBase(AddressingMode::Position));
    }
}
