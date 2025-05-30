use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use url::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to open
    url: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    default_profile: Option<String>,
    #[serde(default)]
    profiles: Vec<ProfileMapping>,
    chrome_path: Option<String>, // Optional: specify custom chrome path
}

#[derive(Deserialize, Debug, Clone)]
struct ProfileMapping {
    name: String,
    profile_directory: String,
    url_patterns: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = std::env::current_exe()?
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("config.toml"); // Expect config.toml in the same directory as the executable or use a fixed path like ~/.config/profile_switcher/config.toml

    // For development, you might want to load it from the project root
    let config_path = if config_path.exists() {
        config_path
    } else {
        PathBuf::from("config.toml") // Fallback for development
    };

    if !config_path.exists() {
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
    let config: Config = toml::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;

    let parsed_url = Url::parse(&args.url)
        .with_context(|| format!("Failed to parse URL: {}", args.url))?;
    let host = parsed_url.host_str().unwrap_or_default();

    let mut selected_profile_dir = config.default_profile.clone();

    for mapping in &config.profiles {
        for pattern in &mapping.url_patterns {
            if host.contains(pattern) || args.url.contains(pattern) {
                selected_profile_dir = Some(mapping.profile_directory.clone());
                println!("Matched profile: {} for URL: {}", mapping.name, args.url);
                break;
            }
        }
        if selected_profile_dir.as_ref() == Some(&mapping.profile_directory) {
            break;
        }
    }

    let profile_to_use = selected_profile_dir
        .as_deref()
        .unwrap_or("Default"); // Fallback to Chrome's default if no specific or default_profile is set

    println!("Using profile directory: {}", profile_to_use);

    // Determine Chrome executable
    let chrome_executable = config.chrome_path.unwrap_or_else(|| {
        // Common paths for Chrome/Chromium on Linux
        if Path::new("/usr/bin/google-chrome-stable").exists() {
            "/usr/bin/google-chrome-stable".to_string()
        } else if Path::new("/usr/bin/google-chrome").exists() {
            "/usr/bin/google-chrome".to_string()
        } else if Path::new("/usr/bin/chromium-browser").exists() {
            "/usr/bin/chromium-browser".to_string()
        } else if Path::new("/usr/bin/chromium").exists() {
            "/usr/bin/chromium".to_string()
        } else {
            // Fallback, user might need to specify in config.toml
            "google-chrome".to_string()
        }
    });

    let mut command = Command::new(chrome_executable);
    command.arg(format!("--profile-directory={}", profile_to_use));
    command.arg(&args.url);

    // Detach the process on Linux/macOS
    #[cfg(not(windows))]
    {
    }

    println!("Executing: {:?} {:?}", command.get_program(), command.get_args());

    match command.spawn() {
        Ok(child) => {
            // Don't wait for the child process to finish for a browser
            // If you need to wait for some reason, use child.wait()
            println!("Chrome launched with URL: {} and profile: {}", args.url, profile_to_use);
            // Detach, if not already handled by spawn on the OS or Chrome itself.
            // For a GUI application like a browser, it's usually best to let it manage its own lifecycle after launch.
            std::mem::forget(child); // Prevent Drop from waiting for the process
        }
        Err(e) => {
            eprintln!("Failed to launch Chrome: {}", e);
            eprintln!("Please ensure Chrome/Chromium is installed and accessible.");
            eprintln!("You can specify the path to Chrome in config.toml using the 'chrome_path' key.");
            std::process::exit(1);
        }
    }

    Ok(())
}
