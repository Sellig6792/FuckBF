use crate::ast::instructions::{InstructionType, InstructionTrait};

#[derive(Clone)]
pub struct OptimizedInstruction {
    pub instruction_type: InstructionType,
    content: Option<Vec<OptimizedInstruction>>,
    pub amount: u32,
}

impl OptimizedInstruction{
    pub fn new(instruction_type: InstructionType, content: Option<Vec<OptimizedInstruction>>) -> OptimizedInstruction {
        OptimizedInstruction {
            instruction_type,
            content,
            amount: 1,
        }
    }

    pub fn add(&mut self, amount: u32) {
        self.amount += amount
    }

    pub fn sub(&mut self, amount: u32) {
        self.amount -= amount
    }

    pub fn is_opposed(&self, other: &OptimizedInstruction) -> bool {
        match self.get_instruction_type() {
            InstructionType::Increment => {
                match other.get_instruction_type() {
                    InstructionType::Decrement => true,
                    _ => false,
                }
            }
            InstructionType::Decrement => {
                match other.get_instruction_type() {
                    InstructionType::Increment => true,
                    _ => false,
                }
            }
            InstructionType::MoveLeft => {
                match other.get_instruction_type() {
                    InstructionType::MoveRight => true,
                    _ => false,
                }
            }
            InstructionType::MoveRight => {
                match other.get_instruction_type() {
                    InstructionType::MoveLeft => true,
                    _ => false,
                }
            }
            InstructionType::MoveLeftScope => {
                match other.get_instruction_type() {
                    InstructionType::MoveRightScope => true,
                    _ => false,
                }
            }
            InstructionType::MoveRightScope => {
                match other.get_instruction_type() {
                    InstructionType::MoveLeftScope => true,
                    _ => false,
                }
            }

            _ => false
        }
    }
}

impl InstructionTrait<OptimizedInstruction> for OptimizedInstruction {
    fn new(instruction_type: InstructionType, content: Option<Vec<OptimizedInstruction>>) -> OptimizedInstruction {
        OptimizedInstruction {
            instruction_type,
            content,
            amount: 1,
        }
    }
    fn get_instruction_type(&self) -> InstructionType {
        self.instruction_type.clone()
    }

    fn get_content(&self) -> Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content.clone(),
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_ref(&self) -> &Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_mut(&mut self) -> &mut Vec<OptimizedInstruction> {
        match &mut self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }

    fn get_amount(&self) -> u32 {
        self.amount
    }
}