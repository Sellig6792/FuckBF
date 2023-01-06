use clap::Parser;

mod cli;
mod fuckbf;

use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if let Some(subcommand) = args.subcommand {
        match subcommand {
            cli::Subcommand::Run(run) => run.run()?,
            cli::Subcommand::Update(update) => update.update()?,
            cli::Subcommand::Version(version) => version.version()?,
        }
    }

    Ok(())
}
