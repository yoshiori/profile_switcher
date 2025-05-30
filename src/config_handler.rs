// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::Result; // Added for Result type used in determine_config_file_path
use std::fs;
use std::path::PathBuf;
use dirs_next;

// Helper function to determine the config path
pub fn determine_config_file_path() -> Result<PathBuf, Vec<PathBuf>> {
    let mut tried_paths = Vec::new();

    // 1. Check ~/.config/profile_switcher/config.toml
    if let Some(mut user_config_dir) = dirs_next::config_dir() {
        user_config_dir.push("profile_switcher");
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
    if let Ok(cwd) = std::env::current_dir() {
        let cwd_config_path = cwd.join("config.toml");
        tried_paths.push(cwd_config_path.clone());
        if cwd_config_path.exists() {
            return Ok(cwd_config_path);
        }
    } else {
        let cwd_config_path_relative = PathBuf::from("config.toml");
        tried_paths.push(cwd_config_path_relative.clone());
        if cwd_config_path_relative.exists() {
            return Ok(cwd_config_path_relative);
        }
    }

    Err(tried_paths)
}

// Helper function to print a standardized "config not found" error message and example
pub fn print_config_not_found_error_and_example(tried_paths: &[PathBuf]) {
    eprintln!("Error: Configuration file \'config.toml\' not found in any of the following locations:");
    for path in tried_paths {
        eprintln!("  - {:?}", path);
    }
    eprintln!("\\nPlease create a config.toml file in one of these locations, or ensure the application has write permissions to create one in ~/.config/profile_switcher/");
    eprintln!("Example config.toml content:");
    eprintln!("{}", include_str!("templates/config.toml"));
}

// Helper function to attempt to create a default config file in XDG config dir
pub fn create_default_config_if_possible() -> Option<PathBuf> {
    if let Some(mut user_config_dir) = dirs_next::config_dir() {
        user_config_dir.push("profile_switcher");
        if let Err(e) = fs::create_dir_all(&user_config_dir) {
            eprintln!("Warning: Could not create config directory at {:?}: {}", user_config_dir, e);
            return None;
        }
        let user_config_file = user_config_dir.join("config.toml");
        let example_config_content = include_str!("templates/config.toml");
        match fs::write(&user_config_file, example_config_content) {
            Ok(_) => {
                eprintln!("Info: Configuration file not found. A new one was created at: {:?}", user_config_file);
                eprintln!("Please review and edit it if necessary. The program will now use this new configuration.");
                Some(user_config_file)
            }
            Err(e) => {
                eprintln!("Warning: Could not write default config file to {:?}: {}", user_config_file, e);
                None
            }
        }
    } else {
        eprintln!("Info: User config directory (e.g., ~/.config) not found. Cannot create default config file automatically.");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path; // Ensure Path is imported for tests

    #[allow(dead_code)]
    fn create_dummy_config_at(path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(path)?;
        writeln!(file, r#"default_profile = "TestDefault""#)?;
        Ok(())
    }

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
        // This test primarily checks determine_config_file_path's error case.
        let unique_dir_name = format!("temp_test_cwd_for_config_not_found_{}",
                                      std::time::SystemTime::now()
                                          .duration_since(std::time::UNIX_EPOCH)
                                          .unwrap().as_nanos());
        let temp_cwd = std::env::temp_dir().join(&unique_dir_name);
        fs::create_dir_all(&temp_cwd).unwrap_or_else(|e| panic!("Failed to create temp CWD {:?}: {}", temp_cwd, e));

        let original_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_cwd).unwrap_or_else(|e| panic!("Failed to set CWD to {:?}: {}", temp_cwd, e));

        // We expect determine_config_file_path to fail here as no config files should exist
        // in the standard locations relative to this temporary CWD or a (typically non-existent for tests)
        // user config dir or exe path.
        let result_determine = determine_config_file_path();

        std::env::set_current_dir(&original_cwd).unwrap_or_else(|e| panic!("Failed to restore CWD to {:?}: {}", original_cwd, e));
        fs::remove_dir_all(&temp_cwd).unwrap_or_else(|e| panic!("Failed to remove temp CWD {:?}: {}", temp_cwd, e));

        assert!(result_determine.is_err(), "Expected an error from determine_config_file_path when no config file is found, but got Ok({:?})", result_determine.ok());

        if let Err(tried_paths) = result_determine {
            let expected_cwd_config_in_temp = temp_cwd.join("config.toml");
            // Check that the CWD path was at least tried
            assert!(
                tried_paths.contains(&expected_cwd_config_in_temp),
                "Expected tried_paths to contain the specific CWD path {:?}, but was {:?}",
                expected_cwd_config_in_temp, tried_paths
            );
            // Also check that some XDG path was likely tried (more robust than specific path string)
            assert!(
                tried_paths.iter().any(|p| p.to_string_lossy().contains(".config") || p.to_string_lossy().contains("profile_switcher")),
                "Expected tried_paths to contain a path related to user config directory, but was {:?}",
                tried_paths
            );
        }
    }

    // It would be beneficial to add tests for create_default_config_if_possible here.
    // This would likely involve:
    // 1. Setting a temporary XDG_CONFIG_HOME environment variable to control where dirs_next::config_dir() points.
    // 2. Running create_default_config_if_possible.
    // 3. Asserting that the file was created in the expected temporary XDG_CONFIG_HOME location with correct content.
    // 4. Cleaning up the temporary directory and file.
    //
    // Example (conceptual, requires careful handling of env vars and temp dirs):
    /*
    #[test]
    fn test_create_default_config_success() {
        let temp_xdg_config_home = std::env::temp_dir().join(format!("temp_xdg_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_xdg_config_home).unwrap();
        let original_xdg_config_home = std::env::var("XDG_CONFIG_HOME");
        std::env::set_var("XDG_CONFIG_HOME", &temp_xdg_config_home);

        let creation_result = create_default_config_if_possible();
        assert!(creation_result.is_some(), "Expected default config creation to succeed");

        if let Some(created_path) = creation_result {
            assert!(created_path.exists(), "Expected config file to exist at {:?}", created_path);
            let content = fs::read_to_string(created_path).unwrap();
            assert!(content.contains("default_profile = \"Default\""));
        }

        // Cleanup
        if let Ok(val) = original_xdg_config_home {
            std::env::set_var("XDG_CONFIG_HOME", val);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        fs::remove_dir_all(temp_xdg_config_home).unwrap();
    }
    */
}
