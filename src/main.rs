mod ast;
mod evaluation;
mod optimization;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

use crate::evaluation::Evaluator;

#[derive(Parser)]
#[command(author, version, about)]
struct CLI {
    // Path is positional argument
    #[arg(required = true, help = "Path to the file to execute")]
    path: std::path::PathBuf,

    #[arg(short = 'O', long = "optimize", help = "Optimize the code", default_value = "false")]
    optimize: bool,
}


fn main() ->  Result<(), Box<dyn std::error::Error>>{
    let args= CLI::parse();
    // Read the file and quit if it's not found
    let program = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.to_str().unwrap()))?;

    // Parse the program
    let mut parser = ast::Parser::new(program);
    let instructions = parser.parse();

    // Optimize the program if "optimize" is true
    if args.optimize {
        let mut optimizer = optimization::Optimizer::new(instructions.clone());
        let optimized_instructions = optimizer.optimize();

        // Evaluate the optimized program
        let mut brainfuck = Evaluator::new(optimized_instructions);
        brainfuck.evaluate(None, None);
    } else {
        // Evaluate the program
        let mut brainfuck = Evaluator::new(instructions);
        brainfuck.evaluate(None, None);
    }

    Ok(())
}
