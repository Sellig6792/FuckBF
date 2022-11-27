use crate::ast::instructions::{Instruction, InstructionType};

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

            input: Vec::new(),
            output_buffer: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, loop_: Option<InstructionType>, show_output: Option<bool>) {
        let instructions = if loop_.is_some() {
            match loop_.unwrap() {
                InstructionType::Loop { instructions } => instructions,
                _ => panic!("Invalid instruction type"),
            }
        } else {
            self.program.clone()
        };

        for instruction in instructions.iter() {
            match &instruction.instruction {
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
                    self.scopes[self.scope_pointer].memory[self.memory_pointer] = self.input.remove(0);
                }
                InstructionType::Output => {
                    self.output_buffer.push(self.scopes[self.scope_pointer].memory[self.memory_pointer]);
                }

                InstructionType::Loop { ref instructions } => {
                    while self.scopes[self.scope_pointer].memory[self.memory_pointer] != 0 {
                        self.evaluate(
                            Some(InstructionType::Loop {
                                instructions: instructions.clone(),
                            }),
                            Some(false),
                        );
                    }
                }
                InstructionType::Function { ref instructions } => {
                    self.scopes[self.scope_pointer].function_memory[self.memory_pointer] = Instruction::new(InstructionType::Function {
                        instructions: instructions.clone(),
                    });
                }

                InstructionType::CallFunction => {
                    match self.scopes[self.scope_pointer].function_memory[self.memory_pointer].instruction.clone() {
                        InstructionType::Function { ref instructions } => {
                            self.scopes.push(Scope::new());
                            self.scope_pointer = self.scopes.len() - 1;
                            self.evaluate(
                                Some(InstructionType::Loop {
                                    instructions: instructions.clone(),
                                }),
                                Some(false),
                            );
                            self.scopes.pop();
                            self.scope_pointer -= 1;
                        }
                        InstructionType::Default => (),
                        _ => panic!("Invalid instruction type in function memory"),
                    }
                }

                InstructionType::MoveLeftScope => {
                    if self.scope_pointer != 0 {
                        self.scope_pointer -= 1;
                    }
                },
                InstructionType::MoveRightScope => {
                    if self.scope_pointer != self.scopes.len() - 1 {
                        self.scope_pointer += 1;
                    }
                },

                _ => (),
            }
        }

        match show_output {
            None => println!("{}", String::from_utf8(self.output_buffer.clone()).unwrap()),
            _ => (),
        }
    }
}
