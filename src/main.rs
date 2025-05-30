use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod profile_selector; // Changed from mod logic;
use profile_selector::{determine_chrome_executable, find_profile_directory, parse_config, Config}; // Changed from use logic::...

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to open
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Config file path logic (remains the same for now, can be refactored too)
    let config_path = std::env::current_exe()?
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("config.toml");

    let config_path = if config_path.exists() {
        config_path
    } else {
        PathBuf::from("config.toml") // Fallback for development
    };

    if !config_path.exists() {
        // Keep the helpful error message for missing config
        eprintln!("Error: Configuration file not found at {:?}.", config_path);
        eprintln!("Please create a config.toml file.");
        eprintln!("Example config.toml:");
        eprintln!("default_profile = \"Default\"");
        eprintln!("chrome_path = \"/usr/bin/google-chrome-stable\" # Or your Chrome/Chromium path");
        eprintln!("");
        eprintln!("[[profiles]]");
        eprintln!("name = \"Work\"");
        eprintln!("profile_directory = \"Profile 1\"");
        eprintln!("url_patterns = [\"github.com\", \"internal.example.com\"]");
        eprintln!("");
        eprintln!("[[profiles]]");
        eprintln!("name = \"Personal\"");
        eprintln!("profile_directory = \"Profile 2\"");
        eprintln!("url_patterns = [\"youtube.com\"]");
        std::process::exit(1);
    }

    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file from {:?}", config_path))?;
    let config: Config = parse_config(&config_str)?;

    let profile_to_use = find_profile_directory(&config, &args.url)?;
    println!("Using profile directory: {}", profile_to_use);

    let chrome_executable = determine_chrome_executable(&config);

    let mut command = Command::new(chrome_executable);
    command.arg(format!("--profile-directory={}", profile_to_use));
    command.arg(&args.url);

    #[cfg(not(windows))]
    {
        // Detach logic can remain here or be moved if it becomes more complex
    }

    println!("Executing: {:?} {:?}", command.get_program(), command.get_args());

    match command.spawn() {
        Ok(child) => {
            println!(
                "Chrome launched with URL: {} and profile: {}",
                args.url,
                profile_to_use
            );
            std::mem::forget(child);
        }
        Err(e) => {
            eprintln!("Failed to launch Chrome: {}", e);
            eprintln!("Please ensure Chrome/Chromium is installed and accessible.");
            eprintln!(
                "You can specify the path to Chrome in config.toml using the 'chrome_path' key."
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
