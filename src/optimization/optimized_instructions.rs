use std::fmt;

use crate::ast::instructions::{Instruction, InstructionType};

#[derive(Clone)]
pub struct OptimizedInstruction {
    pub instruction: Instruction,
    content: Option<Vec<OptimizedInstruction>>,
    pub amount: u32,
}

impl OptimizedInstruction {
    pub fn from(
        instruction: Instruction,
        content: Option<Vec<OptimizedInstruction>>,
    ) -> OptimizedInstruction {
        OptimizedInstruction {
            instruction,
            content,
            amount: 1,
        }
    }

    pub fn get_content(&self) -> Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content.clone(),
            None => vec![],
        }
    }
    pub fn get_content_ref(&self) -> &Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }

    pub fn add(&mut self, amount: u32) {
        self.amount += amount
    }

    pub fn sub(&mut self, amount: u32) {
        self.amount -= amount
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn is_opposed(&self, other: &OptimizedInstruction) -> bool {
        match self.instruction.instruction_type {
            InstructionType::Increment => {
                match other.instruction.instruction_type {
                    InstructionType::Decrement => true,
                    _ => false,
                }
            }
            InstructionType::Decrement => {
                match other.instruction.instruction_type {
                    InstructionType::Increment => true,
                    _ => false,
                }
            }
            InstructionType::MoveLeft => {
                match other.instruction.instruction_type {
                    InstructionType::MoveRight => true,
                    _ => false,
                }
            }
            InstructionType::MoveRight => {
                match other.instruction.instruction_type {
                    InstructionType::MoveLeft => true,
                    _ => false,
                }
            }
            InstructionType::MoveLeftScope => {
                match other.instruction.instruction_type {
                    InstructionType::MoveRightScope => true,
                    _ => false,
                }
            }
            InstructionType::MoveRightScope => {
                match other.instruction.instruction_type {
                    InstructionType::MoveLeftScope => true,
                    _ => false,
                }
            }

            _ => false
        }
    }
}

impl fmt::Debug for OptimizedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instruction.instruction_type {
            InstructionType::Increment => write!(f, "Increment({})", self.amount),
            InstructionType::Decrement => write!(f, "Decrement({})", self.amount),

            InstructionType::MoveLeft => write!(f, "MoveLeft({})", self.amount),
            InstructionType::MoveRight => write!(f, "MoveRight({})", self.amount),

            InstructionType::Input => write!(f, "Input"),
            InstructionType::Output => write!(f, "Output"),

            InstructionType::Loop => write!(f, "Loop{:?}", self.get_content_ref()),

            InstructionType::Function => write!(f, "Function{:?}", self.get_content_ref()),
            InstructionType::CallFunction => write!(f, "CallFunction"),

            InstructionType::MoveLeftScope => write!(f, "MoveLeftScope"),
            InstructionType::MoveRightScope => write!(f, "MoveRightScope"),

            InstructionType::Random => write!(f, "Random"),
        }
    }
}
