use crate::ast::instructions::Instruction;
use crate::ast::instructions::InstructionType::*;

pub struct Parser {
    program: String,
}

impl Parser {
    pub fn new(program: String) -> Parser {
        let program = program.replace(' ', "").replace('\t', "").replace('\r', "").replace('\n', "");

        Parser { program }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let (instructions, _) = self._parse(None, None);
        return instructions;
    }
    fn _parse(&self, index: Option<usize>, stop_char: Option<char>) -> (Vec<Instruction>, usize) {
        let mut index = index.unwrap_or(0);
        let mut instructions = Vec::new();

        while index < self.program.len() {
            let char = match self.program.chars().nth(index) {
                Some(char) => char,
                None => return (instructions, index),
            };
            if stop_char.is_some() && char == stop_char.unwrap() {
                return (instructions, index);
            }

            match char {
                '+' => instructions.push(Instruction::new(Increment)),
                '-' => instructions.push(Instruction::new(Decrement)),

                '<' => instructions.push(Instruction::new(MoveLeft)),
                '>' => instructions.push(Instruction::new(MoveRight)),

                '.' => instructions.push(Instruction::new(Output)),
                ',' => instructions.push(Instruction::new(Input)),

                '[' => {
                    let (loop_instructions, new_index) = self._parse(Some(index + 1), Some(']'));
                    instructions.push(Instruction::new(Loop { instructions: loop_instructions }));
                    index = new_index;
                }
                '{' => {
                    let (function_instructions, new_index) = self._parse(Some(index + 1), Some('}'));
                    instructions.push(Instruction::new(Function { instructions: function_instructions }));
                    index = new_index;
                }

                '=' => instructions.push(Instruction::new(CallFunction)),

                '´' => instructions.push(Instruction::new(MoveLeftScope)),
                '`' => instructions.push(Instruction::new(MoveRightScope)),
                _ => {}
            }

            index += 1;
        }

        return (instructions, index);
    }
}
