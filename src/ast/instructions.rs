use std::fmt;

pub enum InstructionType {
    Increment,
    Decrement,

    MoveLeft,
    MoveRight,

    Input,
    Output,

    Loop { instructions: Vec<Instruction> },
}

pub struct Instruction {
    pub instruction: InstructionType,
}

impl Instruction {
    pub fn new(instruction: InstructionType) -> Instruction {
        Instruction { instruction }
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Instruction {
            instruction: match self.instruction {
                InstructionType::Increment => InstructionType::Increment,
                InstructionType::Decrement => InstructionType::Decrement,

                InstructionType::MoveLeft => InstructionType::MoveLeft,
                InstructionType::MoveRight => InstructionType::MoveRight,

                InstructionType::Input => InstructionType::Input,
                InstructionType::Output => InstructionType::Output,

                InstructionType::Loop { ref instructions } => InstructionType::Loop {
                    instructions: instructions.clone(),
                },
            },
        }
    }
}


impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.instruction {
            InstructionType::Increment => write!(f, "Increment"),
            InstructionType::Decrement => write!(f, "Decrement"),

            InstructionType::MoveLeft => write!(f, "MoveLeft"),
            InstructionType::MoveRight => write!(f, "MoveRight"),

            InstructionType::Input => write!(f, "Input"),
            InstructionType::Output => write!(f, "Output"),

            InstructionType::Loop { ref instructions } => {
                write!(f, "Loop{:?} ", instructions)
            }
        }
    }
}


impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
