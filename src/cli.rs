use std::io::Write;
use std::{env, fs, path};

use anyhow::Context;
use clap;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;

use crate::fuckbrainfuck::evaluation::Evaluator;
use crate::fuckbrainfuck::*;

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

// This macro returns the github api url for the given repository
macro_rules! get_api_url {
    ($repo_url:expr) => {
        format!(
            "https://api.github.com/repos/{}",
            $repo_url.replace("https://github.com/", "")
        )
    };
}

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
    fn get_binary_name() -> String {
        String::from(match env!("FUCKBRAINFUCK_TARGET_OS") {
            "aarch64-unknown-linux-gnu" => "fbf-arm64",
            "i686-unknown-linux-gnu" => "fbf-linux-i686",
            "x86_64-unknown-linux-gnu" => "fbf-linux-x86_64",
            "x86_64-apple-darwin" => "fbf-macos",
            "i686-pc-windows-gnu" => "fbf-win-i686.exe",
            "x86_64-pc-windows-gnu" => "fbf-win-x86_64.exe",
            _ => panic!(
                "No precompiled binary for this target, please compile from source.\
                For more information,see {}#2-building-from-source",
                env!("CARGO_PKG_HOMEPAGE")
            ),
        })
    }

    fn get_api_json(endpoint: &str) -> Value {
        let url = get_api_url!(env!("CARGO_PKG_REPOSITORY"));

        let headers = {
            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static(env!("CARGO_PKG_NAME")));
            headers
        };
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let response = match client.get(&format!("{}/{}", url, endpoint)).send() {
            Ok(response) => response,
            Err(error) => panic!("Could not get latest release: {}", error),
        };

        let json: Value = match response.text() {
            Ok(text) => match serde_json::from_str(&text) {
                Ok(json) => json,
                Err(_) => panic!("Could not parse the response from the github api {}", text),
            },
            Err(_) => panic!("Could not read the response from the github api"),
        };

        json
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let binding = Self::get_api_json("releases/latest");
        let assets = binding.get("assets").unwrap().as_array().unwrap();

        let download_url: String = assets
            .iter()
            .find(|asset| asset.get("name").unwrap().as_str().unwrap() == Self::get_binary_name())
            .unwrap()
            .get("browser_download_url")
            .unwrap()
            .to_string()
            .replace("\"", "");

        println!(
            " {} latest version ({})",
            "Downloading".green().bold(),
            download_url
        );

        let binary = reqwest::blocking::get(&download_url)
            .unwrap()
            .bytes()
            .unwrap();

        println!("  {} latest version", "Installing".green().bold());

        let path = env::current_dir().expect("Could not get the current directory");
        let path = path.join("new-fbf");

        // Write the binary to the current directory
        let mut file = fs::File::create(&path).expect("Could not create the file");
        file.write_all(&binary)
            .expect("Could not write to the file");

        // Move the new binary to the current directory
        fs::rename(&path, env::current_exe().unwrap())
            .expect("Could not move the new binary to the current directory");

        println!("    {} latest version", "Updated".green().bold());

        Ok(())
    }
}

impl Version {
    pub fn version(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("FuckBrainfuck {}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
