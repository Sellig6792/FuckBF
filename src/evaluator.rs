use crate::ast::instructions::{Instruction, InstructionType};

pub struct Evaluator {
    program: Vec<Instruction>,

    memory: [u8; 30000],
    memory_pointer: usize,

    input: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl Evaluator {
    pub fn new(instructions: Vec<Instruction>) -> Evaluator {
        Evaluator {
            program: instructions,

            memory: [0; 30000],
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
            match instruction.instruction {
                InstructionType::Increment => {
                    if self.memory[self.memory_pointer] < 255 {
                        self.memory[self.memory_pointer] += 1
                    } else {
                        self.memory[self.memory_pointer] = 0;
                    }
                }
                InstructionType::Decrement => {
                    if self.memory[self.memory_pointer] > 0 {
                        self.memory[self.memory_pointer] -= 1
                    } else {
                        self.memory[self.memory_pointer] = 255;
                    }
                }

                InstructionType::MoveLeft => {
                    if self.memory_pointer > 0 {
                        self.memory_pointer -= 1;
                    } else {
                        self.memory_pointer = 29999;
                    }
                }
                InstructionType::MoveRight => {
                    if self.memory_pointer < 29999 {
                        self.memory_pointer += 1;
                    } else {
                        self.memory_pointer = 0;
                    }
                }

                InstructionType::Input => {
                    if self.input.is_empty() {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        // Convert the input to a vector of u8
                        self.input = input.trim().bytes().collect();
                    }
                    self.memory[self.memory_pointer] = self.input.remove(0);
                }
                InstructionType::Output => {
                    self.output_buffer.push(self.memory[self.memory_pointer])
                }

                InstructionType::Loop { ref instructions } => {
                    while self.memory[self.memory_pointer] != 0 {
                        self.evaluate(
                            Some(InstructionType::Loop {
                                instructions: instructions.clone(),
                            }),
                            Some(false),
                        );
                    }
                }
            }
        }

        if show_output.unwrap_or(true) {
            println!("{}", String::from_utf8(self.output_buffer.clone()).unwrap());
        }
    }
}
