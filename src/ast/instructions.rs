use std::fmt;

#[derive(Debug, Clone, Default)]
pub enum InstructionType {
    Increment,
    Decrement,

    MoveLeft,
    MoveRight,

    Input,
    Output,

    Loop { instructions: Vec<Instruction> },

    Function { instructions: Vec<Instruction> },
    CallFunction,

    MoveLeftScope,
    MoveRightScope,

    #[default]
    Default,
}

#[derive(Clone, Default)]
pub struct Instruction {
    pub instruction: InstructionType,
}

impl Instruction {
    pub fn new(instruction: InstructionType) -> Instruction {
        Instruction { instruction }
    }
}



impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instruction {
            InstructionType::Increment => write!(f, "Increment"),
            InstructionType::Decrement => write!(f, "Decrement"),

            InstructionType::MoveLeft => write!(f, "MoveLeft"),
            InstructionType::MoveRight => write!(f, "MoveRight"),

            InstructionType::Input => write!(f, "Input"),
            InstructionType::Output => write!(f, "Output"),

            InstructionType::Loop { instructions } => {
                write!(f, "Loop{:?}", instructions)
            }

            InstructionType::Function { instructions } => {
                write!(f, "Function{:?}", instructions)
            }
            InstructionType::CallFunction => write!(f, "CallFunction"),

            InstructionType::MoveLeftScope => write!(f, "MoveLeftScope"),
            InstructionType::MoveRightScope => write!(f, "MoveRightScope"),

            InstructionType::Default => write!(f, "Default"),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
