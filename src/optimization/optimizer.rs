use crate::ast::{Instruction, InstructionTrait, InstructionType};

use crate::optimization::optimized_instructions::OptimizedInstruction;

pub struct Optimizer {
    instructions: Vec<Instruction>,
}

impl Optimizer {
    pub fn new(instructions: Vec<Instruction>) -> Optimizer {
        Optimizer { instructions }
    }

    pub fn optimize(&mut self) -> Vec<OptimizedInstruction> {
        let mut optimized_instructions = vec![];
        self.merge_instructions(&mut optimized_instructions);
        self.cancel_opposed_instructions(&mut optimized_instructions);
        optimized_instructions
    }

    fn optimize_container(&self, content: Vec<Instruction>) -> Vec<OptimizedInstruction> {
        let mut optimizer = Optimizer::new(content);
        optimizer.optimize()
    }

    fn merge_instructions(&self, optimized_instructions: &mut Vec<OptimizedInstruction>) -> () {
        for instruction in self.instructions.iter() {
            match optimized_instructions.last_mut() {
                Some(last_optimized_instruction) => {
                    if last_optimized_instruction.get_instruction_type()
                        == instruction.get_instruction_type()
                    {
                        match instruction.get_instruction_type() {
                            InstructionType::Function => {
                                optimized_instructions.pop();
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    Some(self.optimize_container(instruction.get_content())),
                                ));
                            }
                            _ => last_optimized_instruction.add(1)
                        }

                    } else {
                        match instruction.get_instruction_type() {
                            InstructionType::Function | InstructionType::Loop => {
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    Some(
                                        self.optimize_container(instruction.get_content().clone()),
                                    ),
                                ));
                            }
                            _ => {
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    None,
                                ));
                            }
                        }
                    }
                }
                None => match instruction.instruction_type {
                    InstructionType::Function | InstructionType::Loop => {
                        optimized_instructions.push(OptimizedInstruction::new(
                            instruction.get_instruction_type(),
                            Some(self.optimize_container(instruction.get_content().clone())),
                        ));
                    }
                    _ => optimized_instructions.push(OptimizedInstruction::new(
                        instruction.get_instruction_type(),
                        None,
                    )),
                },
            }
        }
    }

    fn cancel_opposed_instructions(
        &self,
        optimized_instructions: &mut Vec<OptimizedInstruction>,
    ) -> () {
        let mut new_optimized_instructions: Vec<OptimizedInstruction> = vec![];

        for optimized_instruction in optimized_instructions.iter() {
            match new_optimized_instructions.last().clone() {
                Some(last_optimized_instruction) => {
                    if last_optimized_instruction.is_opposed(optimized_instruction) {
                        let last_amount = last_optimized_instruction.get_amount();
                        let current_amount = optimized_instruction.get_amount();

                        if last_amount > current_amount {
                            new_optimized_instructions
                                .last_mut()
                                .expect("Error while getting last optimized instruction")
                                .sub(current_amount);
                        } else if last_amount < current_amount {
                            new_optimized_instructions.pop();
                            new_optimized_instructions.push(OptimizedInstruction::new(
                                optimized_instruction.get_instruction_type(),
                                None,
                            ));
                            new_optimized_instructions
                                .last_mut()
                                .expect("Error while getting last optimized instruction")
                                .set_amount(current_amount - last_amount);
                        } else {
                            new_optimized_instructions.pop();
                        }
                    } else if last_optimized_instruction.get_instruction_type()
                        == optimized_instruction.get_instruction_type()
                    {
                        // If the current instruction is the same as the last one, we add the amount of the current instruction to the last one
                        new_optimized_instructions
                            .last_mut()
                            .expect(
                                "Error while getting last element of new_optimized_instructions",
                            )
                            .add(optimized_instruction.amount);
                    } else {
                        new_optimized_instructions.push(optimized_instruction.clone());
                    }
                }

                None => new_optimized_instructions.push(optimized_instruction.clone()),
            }
        }

        *optimized_instructions = new_optimized_instructions;
    }
}
