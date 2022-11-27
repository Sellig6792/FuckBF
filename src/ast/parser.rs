use crate::ast::instructions::Instruction;
use crate::ast::instructions::InstructionType::*;

pub struct Parser {
    program: String,
    index: usize,
}

impl Parser {
    pub fn new(program: String) -> Parser {
        Parser { program, index: 0 }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while self.index < self.program.len() {
            match self.program.chars().nth(self.index).unwrap() {
                '>' => instructions.push(Instruction::new(MoveRight)),
                '<' => instructions.push(Instruction::new(MoveLeft)),
                '+' => instructions.push(Instruction::new(Increment)),
                '-' => instructions.push(Instruction::new(Decrement)),
                '.' => instructions.push(Instruction::new(Output)),
                ',' => instructions.push(Instruction::new(Input)),
                '[' => {
                    let (loop_instructions, index) = self.parse_loop();
                    self.index = index;
                    instructions.push(Instruction::new(Loop {
                        instructions: loop_instructions,
                    }));
                }
                ']' => break,
                _ => (),
            }

            self.index += 1;
        }

        instructions
    }

    fn parse_loop(&self) -> (Vec<Instruction>, usize) {
        let mut index = self.index + 1;
        let mut loop_count = 1;

        while index < self.program.len() {
            match self.program.chars().nth(index).unwrap() {
                '[' => loop_count += 1,
                ']' => loop_count -= 1,
                _ => (),
            }

            if loop_count == 0 {
                break;
            }

            index += 1;
        }

        let loop_instructions = self.program[self.index + 1..index].to_string();
        let mut parser = Parser::new(loop_instructions);

        (parser.parse(), index)
    }
}
