mod ast;
mod evaluation;
mod optimization;

use std::fs;

use crate::evaluation::Evaluator;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Get path and quit if it's not provided
    let path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Please provide a path to a file");
            return;
        }
    };

    // Read the file and quit if it's not found
    let program = if fs::metadata(&path).is_ok() {
        fs::read_to_string(&path).expect("Unable to read file")
    } else {
        println!("File not found");
        return;
    };

    // Parse the program
    let mut parser = ast::Parser::new(program);
    let instructions = parser.parse();

    // Optimize the program
    let mut optimizer = optimization::Optimizer::new(instructions.clone());
    let optimized_instructions = optimizer.optimize();

    // Run the program
    let mut brainfuck = Evaluator::new(instructions.clone());
    brainfuck.evaluate(None, None);
}
