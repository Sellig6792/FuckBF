use std::{fs, path};

use anyhow::Context;
use clap::{arg, Parser};
use colored::Colorize;

use crate::fuckbf::{evaluation::Evaluator, *};

#[derive(Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
pub struct Cli {
    // Path
    #[arg(required = false, help = "Path to the file to execute")]
    pub path: Option<path::PathBuf>,

    // Optimize flag (if path is given)
    #[arg(
        short = 'O',
        long = "optimize",
        help = "Optimize the code before executing it",
        default_value = "false"
    )]
    pub optimize: bool,

    // Update option (if no path is given)
    #[arg(short = 'U', long = "update", help = "Update the program")]
    pub update: bool,
}

pub fn run(path: &path::PathBuf, optimize: bool) -> Result<(), Box<dyn std::error::Error>> {
    let program = fs::read_to_string(path)
        .with_context(|| format!("Could not read file: {}", path.display()))?;

    // Parse the program
    let mut parser = ast::Parser::new(program);
    let instructions = parser.parse();

    // Optimize the program if "optimize" is true
    if optimize {
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

pub mod update {
    use super::Colorize;
    use anyhow::Context;
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
    use serde_json::Value;
    use std::io::Write;
    use std::{env, fs};

    // This macro returns the github api url for the given repository
    macro_rules! get_api_url {
        ($repo_url:expr) => {
            format!(
                "https://api.github.com/repos/{}",
                $repo_url.replace("https://github.com/", "")
            )
        };
    }

    fn get_binary_name() -> String {
        String::from(match env!("FUCKBF_TARGET_OS") {
            "aarch64-unknown-linux-gnu" => "fuckbf-arm64",
            "i686-unknown-linux-gnu" => "fuckbf-linux-i686",
            "x86_64-unknown-linux-gnu" => "fbf-linux-x86_64",
            "x86_64-apple-darwin" => "fuckbf-macos",
            "i686-pc-windows-gnu" => "fuckbf-win-i686.exe",
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

    pub fn update() -> Result<(), Box<dyn std::error::Error>> {
        let binding_current_exe = env::current_exe().unwrap();
        let installation_dir = binding_current_exe.parent().unwrap();
        let binding_api_latest_release = get_api_json("releases/latest");
        let assets = binding_api_latest_release
            .get("assets")
            .unwrap()
            .as_array()
            .unwrap();

        let download_url: String = assets
            .iter()
            .find(|asset| asset.get("name").unwrap().as_str().unwrap() == get_binary_name())
            .with_context(|| {
                format!(
                    "No precompiled binary for this target, please compile from source.\
            For more information,see {}#2-building-from-source",
                    env!("CARGO_PKG_HOMEPAGE")
                )
            })?
            .get("browser_download_url")
            .with_context(|| {
                format!(
                    "No precompiled binary for this target, please compile from source.\
            For more information,see {}#2-building-from-source",
                    env!("CARGO_PKG_HOMEPAGE")
                )
            })?
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

        let path = installation_dir.join("fuckbf.new");

        // Write the binary to the current directory
        let mut file = fs::File::create(&path).expect("Could not create the new binary");
        file.write_all(&binary)
            .expect("Could not write to the file");

        let current_binary = env::current_exe().unwrap();

        // Move current binary to a .old file
        fs::rename(&current_binary, installation_dir.join("fuckbf.old"))
            .expect("Could not rename the current binary");

        // Move the new binary to the current binary
        fs::rename(&path, &current_binary).expect("Could not rename the new binary");

        println!("     {} latest version", "Updated".green().bold());

        Ok(())
    }

    // Delete the {}.old file if it exists it was generated by a previous update (the file is in the same directory as the executable)
    pub fn delete_old_file() {
        let mut old_file_path = env::current_exe().unwrap();
        old_file_path.pop();
        old_file_path.push("fuckbf.old");

        if old_file_path.exists() {
            fs::remove_file(&old_file_path).expect("Could not delete old file");
        }
    }
}
