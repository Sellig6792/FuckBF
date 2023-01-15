use clap::Parser;
use colored::Colorize;

mod cli;
mod fuckbf;

use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if let Some(path) = args.path {
        cli::run(&path, args.optimize)?;
    } else {
        println!("{} The following required arguments were not provided:\n  {}\n\n{} {} [OPTIONS] [PATH]\n\nFor more information try {}",
                 "error:".red().bold(),
                 "PATH".green(),
                 "Usage:".bold().underline(),
                 "fuckbf.exe".bold(),
                 "'--help'".bold()
        );
    }
    Ok(())
}
