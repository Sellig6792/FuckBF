use std::{fs, path};
use anyhow::Context;

use clap;

use crate::fuckbrainfuck::*;
use crate::fuckbrainfuck::evaluation::Evaluator;


#[derive(clap::Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,
}


#[derive(clap::Subcommand)]
pub enum Subcommand {
    #[clap(about = "Run the given FuckBrainfuck program")]
    Run(Run),

    #[clap(about = "Update FuckBrainfuck to the latest version")]
    Update(Update),


    #[clap(about = "Prints the version of the program")]
    Version(Version),
}

#[derive(clap::Parser)]
pub struct Run {
    #[arg(required = true, help = "Path to the file to execute")]
    pub path: path::PathBuf,

    #[clap(short, long, help = "Optimize the program before running it")]
    pub optimize: bool,
}


#[derive(clap::Parser)]
pub struct Update {}

#[derive(clap::Parser)]
pub struct Version {}




impl Run {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let program = fs::read_to_string(&self.path)
            .with_context(|| format!("Could not read file: {}", self.path.display()))?;

        // Parse the program
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();

        // Optimize the program if "optimize" is true
        if self.optimize {
            let mut optimizer = optimization::Optimizer::new(instructions);
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
}


impl Update {
    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}




impl Version {
    pub fn version(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}