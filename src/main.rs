// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf}; // Path is used by determine_config_file_path and tests
use std::process::Command;

mod profile_selector;
use profile_selector::{determine_chrome_executable, find_profile_directory, parse_config, Config};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to open
    url: String,
}

// Helper function to determine the config path
fn determine_config_file_path() -> Result<PathBuf, Vec<PathBuf>> {
    let mut tried_paths = Vec::new();

    // 1. Check ~/.config/profile_switcher/config.toml
    if let Some(mut user_config_dir) = dirs_next::config_dir() {
        user_config_dir.push("profile_switcher");
        // It's good practice to create the directory if it doesn't exist,
        // though for reading, we only care if the file exists.
        // fs::create_dir_all(&user_config_dir).ok(); // Optional: create if not exists
        let user_config_file = user_config_dir.join("config.toml");
        tried_paths.push(user_config_file.clone());
        if user_config_file.exists() {
            return Ok(user_config_file);
        }
    }

    // 2. Check next to executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_config_path = exe_dir.join("config.toml");
            tried_paths.push(exe_config_path.clone());
            if exe_config_path.exists() {
                return Ok(exe_config_path);
            }
        }
    }

    // 3. Check current working directory
    // Use current_dir() which returns an absolute path.
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_config_path = cwd.join("config.toml");
        tried_paths.push(cwd_config_path.clone());
        if cwd_config_path.exists() {
            return Ok(cwd_config_path);
        }
    } else { // Fallback if current_dir() fails (less likely but good to handle)
        let cwd_config_path_relative = PathBuf::from("config.toml");
        tried_paths.push(cwd_config_path_relative.clone());
        if cwd_config_path_relative.exists() { // This checks relative to current CWD
            return Ok(cwd_config_path_relative);
        }
    }
    
    Err(tried_paths)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = match determine_config_file_path() {
        Ok(path) => path,
        Err(tried_paths) => {
            eprintln!("Error: Configuration file 'config.toml' not found in any of the following locations:");
            for path in tried_paths {
                eprintln!("  - {:?}", path);
            }
            eprintln!("\nPlease create a config.toml file in one of these locations.");
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
    };

    println!("Using config file: {:?}", config_path); // Keep for now, useful for user

    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file from {:?}", config_path))?;
    let config: Config = parse_config(&config_str)?;

    let profile_to_use = find_profile_directory(&config, &args.url)?;
    // The println! in find_profile_directory is for debugging, consider removing or making it conditional
    // println!("Using profile directory: {}", profile_to_use); 

    let chrome_executable = determine_chrome_executable(&config);

    let mut command = Command::new(chrome_executable);
    command.arg(format!("--profile-directory={}", profile_to_use));
    command.arg(&args.url);

    // Detach logic for non-Windows can remain if needed, or be platform-specific.
    // #[cfg(not(windows))]
    // {
    // }

    // println!("Executing: {:?} {:?}", command.get_program(), command.get_args()); // Debugging

    match command.spawn() {
        Ok(child) => {
            // println!( // Debugging
            //     "Chrome launched with URL: {} and profile: {}",
            //     args.url,
            //     profile_to_use
            // );
            std::mem::forget(child); // Detach the process
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

#[cfg(test)]
mod tests {
    use super::*; // Brings determine_config_file_path into scope
    use std::fs::{self, File};
    use std::io::Write;
    // PathBuf is not directly used, Path is used by create_dummy_config_at
    // use std::path::PathBuf; 

    // Helper to create a dummy config file
    #[allow(dead_code)] 
    fn create_dummy_config_at(path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        // Use raw string literal to avoid escaping issues with quotes
        writeln!(file, r#"default_profile = "TestDefault""#)?;
        Ok(())
    }
    
    // Helper to remove a file and its parent if the parent was created by the test
    #[allow(dead_code)] 
    fn cleanup_dummy_file(path: &Path, created_parent_dir: Option<&Path>) {
        fs::remove_file(path).ok();
        if let Some(parent_dir) = created_parent_dir {
            if parent_dir.exists() && parent_dir.is_dir() { 
                 fs::remove_dir(parent_dir).ok(); 
            }
        }
    }

    #[test]
    fn test_config_not_found_when_no_files_exist() {
        // This test relies on the test environment being clean or the paths being unique enough.
        // It simulates a scenario where no config file is present in any of the expected locations.

        let unique_dir_name = format!("temp_test_cwd_for_not_found_{}", 
                                      std::time::SystemTime::now()
                                          .duration_since(std::time::UNIX_EPOCH)
                                          .unwrap().as_nanos());
        let temp_cwd = std::env::temp_dir().join(&unique_dir_name);
        fs::create_dir_all(&temp_cwd).unwrap_or_else(|e| panic!("Failed to create temp CWD {:?}: {}", temp_cwd, e));
        
        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_cwd).unwrap_or_else(|e| panic!("Failed to set CWD to {:?}: {}", temp_cwd, e));

        // At this point, we assume:
        // 1. ~/.config/profile_switcher/config.toml does not exist (or dirs_next::config_dir() is None).
        // 2. The dummy executable for tests (if any) won't have a config.toml next to it.
        // 3. Our temp_cwd is empty.

        let result = determine_config_file_path();
        
        std::env::set_current_dir(&original_cwd).unwrap_or_else(|e| panic!("Failed to restore CWD to {:?}: {}", original_cwd, e));
        fs::remove_dir_all(&temp_cwd).unwrap_or_else(|e| panic!("Failed to remove temp CWD {:?}: {}", temp_cwd, e));

        assert!(result.is_err(), "Expected an error when no config file is found, but got Ok({:?})", result.ok());
        
        if let Err(tried_paths) = result {
            let expected_cwd_config_in_temp = temp_cwd.join("config.toml");
            assert!(
                tried_paths.contains(&expected_cwd_config_in_temp),
                "Expected tried_paths to contain the specific CWD path {:?}, but was {:?}",
                expected_cwd_config_in_temp, tried_paths
            );
        }
    }

    // To add more comprehensive tests for scenarios where config files *do* exist in specific locations
    // (e.g., user config, next to exe, or CWD), you would typically need:
    // 1. A way to mock or control the paths returned by `dirs_next::config_dir()` and `std::env::current_exe()`.
    // 2. A way to create dummy config files at these controlled locations specifically for the test.
    // 3. Ensure proper cleanup of these dummy files and directories after each test.
    //
    // Without a mocking library or further refactoring `determine_config_file_path`
    // to inject these dependencies (e.g., passing them as arguments),
    // creating fully isolated and reliable unit tests for each specific path-found scenario is complex.
    // The `test_config_not_found_when_no_files_exist` provides some basic coverage for the error path.
}
