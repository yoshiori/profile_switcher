// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::process::Command;

mod profile_selector;
use profile_selector::{determine_chrome_executable_with_profile, find_matching_profile, parse_config, Config};

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

    let matched_profile = find_matching_profile(&config, &args.url)?;
    
    // Determine profile and chrome executable
    let (profile_to_use, chrome_executable) = if let Some(profile) = &matched_profile {
        // URL matched a specific profile
        (profile.profile_directory.clone(), determine_chrome_executable_with_profile(&config, Some(profile)))
    } else if let Some(default_profile_name) = &config.default_profile {
        // No match, use default profile
        let default_profile = config.profiles.iter().find(|p| &p.name == default_profile_name);
        match default_profile {
            Some(profile) => (profile.profile_directory.clone(), determine_chrome_executable_with_profile(&config, Some(profile))),
            None => {
                eprintln!("Warning: Default profile '{}' not found in configuration. Using fallback.", default_profile_name);
                ("Default".to_string(), determine_chrome_executable_with_profile(&config, None))
            }
        }
    } else {
        // No match and no default profile configured
        ("Default".to_string(), determine_chrome_executable_with_profile(&config, None))
    };

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
