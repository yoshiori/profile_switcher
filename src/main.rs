// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::process::Command;

mod profile_selector;
use profile_selector::{determine_chrome_executable, find_profile_directory, parse_config, Config};

mod config_handler;
use config_handler::{determine_config_file_path, create_default_config_if_possible, print_config_not_found_error_and_example};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to open
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = match determine_config_file_path() {
        Ok(path) => path,
        Err(tried_paths) => {
            match create_default_config_if_possible() {
                Some(created_path) => created_path,
                None => {
                    print_config_not_found_error_and_example(&tried_paths);
                    return Err(anyhow::anyhow!("Configuration file not found and could not be created. Please check permissions or create the file manually."));
                }
            }
        }
    };

    println!("Using config file: {:?}", config_path);

    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file from {:?}", config_path))?;
    let config: Config = parse_config(&config_str)?;

    let profile_to_use = find_profile_directory(&config, &args.url)?;

    let chrome_executable = determine_chrome_executable(&config);

    let mut command = Command::new(chrome_executable);
    command.arg(format!("--profile-directory={}", profile_to_use));
    command.arg(&args.url);

    match command.spawn() {
        Ok(child) => {
            std::mem::forget(child);
        }
        Err(e) => {
            eprintln!("Failed to launch Chrome: {}", e);
            eprintln!("Please ensure Chrome/Chromium is installed and accessible.");
            eprintln!(
                "You can specify the path to Chrome in config.toml using the 'chrome_path' key."
            );
            return Err(e.into());
        }
    }

    Ok(())
}
