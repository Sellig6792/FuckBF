use rand::Rng;

use crate::ast::instructions::{Instruction, InstructionType};
use crate::optimization::optimized_instructions::OptimizedInstruction;

use crate::evaluation::Scope;

pub struct Evaluator {
    program: Vec<Instruction>,

    scopes: Vec<Scope>,
    scope_pointer: usize,

    memory_pointer: usize,
    input: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl Evaluator {
    pub fn new(instructions: Vec<Instruction>) -> Evaluator {
        Evaluator {
            program: instructions,

            scopes: vec![Scope::new()],

            scope_pointer: 0,

            memory_pointer: 0,

            input: vec![],
            output_buffer: vec![],
        }
    }

    pub fn evaluate(
        &mut self,
        container_to_execute: Option<Instruction>,
        show_output: Option<bool>,
    ) {
        let instructions = match container_to_execute {
            Some(container) => container.get_content(),
            None => self.program.clone(),
        };

        for instruction in instructions.iter() {
            match &instruction.instruction_type {
                InstructionType::Increment => {
                    if self.scopes[self.scope_pointer].memory[self.memory_pointer] == 255 {
                        self.scopes[self.scope_pointer].memory[self.memory_pointer] = 0;
                    } else {
                        self.scopes[self.scope_pointer].memory[self.memory_pointer] += 1;
                    }
                }
                InstructionType::Decrement => {
                    if self.scopes[self.scope_pointer].memory[self.memory_pointer] == 0 {
                        self.scopes[self.scope_pointer].memory[self.memory_pointer] = 255;
                    } else {
                        self.scopes[self.scope_pointer].memory[self.memory_pointer] -= 1;
                    }
                }

                InstructionType::MoveLeft => {
                    if self.memory_pointer == 0 {
                        self.memory_pointer = 29999;
                    } else {
                        self.memory_pointer -= 1;
                    }
                }
                InstructionType::MoveRight => {
                    if self.memory_pointer == 29999 {
                        self.memory_pointer = 0;
                    } else {
                        self.memory_pointer += 1;
                    }
                }

                InstructionType::Input => {
                    if self.input.is_empty() {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        // Convert the input to a vector of u8
                        self.input = input.trim().bytes().collect();
                    }
                    self.scopes[self.scope_pointer].memory[self.memory_pointer] =
                        self.input.remove(0);
                }
                InstructionType::Output => {
                    self.output_buffer
                        .push(self.scopes[self.scope_pointer].memory[self.memory_pointer]);
                }

                InstructionType::Loop => {
                    while self.scopes[self.scope_pointer].memory[self.memory_pointer] != 0 {
                        self.evaluate(Some(instruction.clone()), Some(false))
                    }
                }
                InstructionType::Function => {
                    self.scopes[self.scope_pointer].function_memory[self.memory_pointer] =
                        Instruction::new(
                            InstructionType::Function,
                            Some(instruction.get_content()),
                        );
                }

                InstructionType::CallFunction => {
                    self.scopes.push(Scope::new());
                    self.scope_pointer += 1;
                    self.evaluate(
                        Some(
                            self.scopes[self.scope_pointer - 1].function_memory
                                [self.memory_pointer]
                                .clone(),
                        ),
                        Some(false),
                    );
                    self.scopes.pop();
                    self.scope_pointer -= 1;
                }

                InstructionType::MoveLeftScope => {
                    if self.scope_pointer != 0 {
                        self.scope_pointer -= 1;
                    }
                }
                InstructionType::MoveRightScope => {
                    if self.scope_pointer != self.scopes.len() - 1 {
                        self.scope_pointer += 1;
                    }
                }

                InstructionType::Random => {
                    /*
                    Generate a random number between the left cell's value and the right cell's value (including both)

                    If the left cell's value is greater than the right cell's value,
                    generate a random number between the left cell's value and 255 and the right cell's value and 0
                     */
                    let left = self.scopes[self.scope_pointer].memory[self.memory_pointer - 1];
                    let right = self.scopes[self.scope_pointer].memory[self.memory_pointer + 1];

                    if right > left {
                        let r = rand::thread_rng().gen_range(left..=right);
                        self.scopes[self.scope_pointer].memory[self.memory_pointer] = r;
                    } else {
                        let left_to_255 = rand::thread_rng().gen_range(left..=255);
                        let _0_to_right = rand::thread_rng().gen_range(0..=right);

                        let left_to_255_or_right_to_0 = rand::thread_rng().gen_range(0..=1);

                        if left_to_255_or_right_to_0 == 0 {
                            self.scopes[self.scope_pointer].memory[self.memory_pointer] =
                                left_to_255;
                        } else {
                            self.scopes[self.scope_pointer].memory[self.memory_pointer] =
                                _0_to_right;
                        }
                    }
                }
            }
        }

        match show_output {
            None => println!("{}", String::from_utf8(self.output_buffer.clone()).unwrap()),
            _ => (),
        }
    }
}

struct OptimizedEvaluator {
    program: Vec<OptimizedInstruction>,

    scopes: Vec<Scope>,
    scope_pointer: usize,

    memory_pointer: usize,
    input: Vec<u8>,
    output_buffer: Vec<u8>,
}

