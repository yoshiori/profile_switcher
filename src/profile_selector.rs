// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct ProfileMapping {
    pub name: String,
    pub profile_directory: String,
    pub url_patterns: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub default_profile: Option<String>,
    #[serde(default)]
    pub profiles: Vec<ProfileMapping>,
    pub chrome_path: Option<String>,
}

pub fn parse_config(config_str: &str) -> Result<Config> {
    toml::from_str(config_str).with_context(|| "Failed to parse TOML config string")
}

pub fn find_profile_directory(config: &Config, url_str: &str) -> Result<String> {
    let parsed_url =
        Url::parse(url_str).with_context(|| format!("Failed to parse URL: {}", url_str))?;
    let host = parsed_url.host_str().unwrap_or_default();

    let mut selected_profile_dir_name: Option<String> = None;
    let mut longest_match_len = 0;

    for mapping in &config.profiles {
        for pattern in &mapping.url_patterns {
            if (host.contains(pattern) || url_str.contains(pattern)) && pattern.len() > longest_match_len {
                selected_profile_dir_name = Some(mapping.profile_directory.clone());
                longest_match_len = pattern.len();
                println!(
                    "Matched profile: {} (directory: {}) with pattern '{}' (len: {}) for URL: {}",
                    mapping.name,
                    mapping.profile_directory,
                    pattern,
                    pattern.len(),
                    url_str
                );
                // We don't break here anymore, we want to find the longest match among all patterns
            }
        }
    }

    Ok(selected_profile_dir_name
        .or_else(|| config.default_profile.clone())
        .unwrap_or_else(|| "Default".to_string())) // Fallback to Chrome's default
}

pub fn determine_chrome_executable(config: &Config) -> String {
    config.chrome_path.clone().unwrap_or_else(|| {
        // Common paths for Chrome/Chromium on Linux
        if Path::new("/usr/bin/google-chrome-stable").exists() {
            "/usr/bin/google-chrome-stable".to_string()
        } else if Path::new("/usr/bin/google-chrome").exists() {
            "/usr/bin/google-chrome".to_string()
        } else if Path::new("/usr/bin/chromium-browser").exists() {
            "/usr/bin/chromium-browser".to_string()
        } else if Path::new("/usr/bin/chromium").exists() {
            "/usr/bin/chromium".to_string()
        }
        // macOS specific path
        else if Path::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome").exists() {
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string()
        }
        // Add Windows paths if necessary, though direct execution might differ.
        // else if Path::new("C:\Program Files\Google\Chrome\Application\chrome.exe").exists() {
        //     "C:\Program Files\Google\Chrome\Application\chrome.exe".to_string()
        // }
        // else if Path::new("C:\Program Files (x86)\Google\Chrome\Application\chrome.exe").exists() {
        //     "C:\Program Files (x86)\Google\Chrome\Application\chrome.exe".to_string()
        // }
        else {
            // Fallback, user might need to specify in config.toml or ensure it's in PATH
            "google-chrome".to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            default_profile: Some("DefaultProfile".to_string()),
            profiles: vec![
                ProfileMapping {
                    name: "Work".to_string(),
                    profile_directory: "Profile 1".to_string(),
                    url_patterns: vec!["github.com".to_string(), "work.example.com".to_string()],
                },
                ProfileMapping {
                    name: "Personal".to_string(),
                    profile_directory: "Profile 2".to_string(),
                    url_patterns: vec!["youtube.com".to_string(), "personal.blog".to_string()],
                },
                ProfileMapping {
                    name: "Shopping".to_string(),
                    profile_directory: "Profile 3".to_string(),
                    url_patterns: vec!["amazon.com".to_string()],
                },
            ],
            chrome_path: None, // Test default path detection or provide one
        }
    }

    #[test]
    fn test_parse_valid_config() {
        let config_content = r#"
            default_profile = "Default"
            chrome_path = "/usr/bin/custom-chrome"

            [[profiles]]
            name = "Work"
            profile_directory = "WorkProfile"
            url_patterns = ["work.com", "project.site"]

            [[profiles]]
            name = "Personal"
            profile_directory = "PersonalProfile"
            url_patterns = ["personal.com"]
        "#;
        let config = parse_config(config_content).unwrap();
        assert_eq!(config.default_profile, Some("Default".to_string()));
        assert_eq!(config.chrome_path, Some("/usr/bin/custom-chrome".to_string()));
        assert_eq!(config.profiles.len(), 2);
        assert_eq!(config.profiles[0].name, "Work");
        assert_eq!(config.profiles[0].profile_directory, "WorkProfile");
    }

    #[test]
    fn test_parse_invalid_config() {
        let config_content = "this is not toml";
        assert!(parse_config(config_content).is_err());
    }

    #[test]
    fn test_find_profile_work() {
        let config = create_test_config();
        let profile = find_profile_directory(&config, "https://github.com/user/repo").unwrap();
        assert_eq!(profile, "Profile 1");

        let profile_direct_match = find_profile_directory(&config, "https://work.example.com/dashboard").unwrap();
        assert_eq!(profile_direct_match, "Profile 1");
    }

    #[test]
    fn test_find_profile_personal() {
        let config = create_test_config();
        let profile = find_profile_directory(&config, "https://www.youtube.com/watch?v=video").unwrap();
        assert_eq!(profile, "Profile 2");
    }

    #[test]
    fn test_find_profile_shopping_direct_url_match() {
        // Test a case where the pattern might be a full URL or a very specific path part
        let mut test_config_specific_url = create_test_config();
        // Ensure the more specific pattern is added.
        // For the test to be robust, the order in `profiles` shouldn't strictly matter if logic is correct.
        // However, to ensure this specific mapping is tested against others, we can add it.
        test_config_specific_url.profiles.insert(0, ProfileMapping { // Insert at the beginning to test against general later
            name: "SpecificDeal".to_string(),
            profile_directory: "DealProfile".to_string(),
            url_patterns: vec!["amazon.com/deals/today".to_string()],
        });

        let profile = find_profile_directory(&test_config_specific_url, "https://www.amazon.com/deals/today?ref=nav").unwrap();
        assert_eq!(profile, "DealProfile");

        // Check that a more general amazon.com still matches "Profile 3" from the original create_test_config()
        // if "SpecificDeal" was not present or did not match.
        let general_amazon_config = create_test_config(); // Use original config for this part of the test
        let general_amazon_profile = find_profile_directory(&general_amazon_config, "https://www.amazon.com/gp/cart/view.html").unwrap();
        assert_eq!(general_amazon_profile, "Profile 3");
    }


    #[test]
    fn test_find_profile_default() {
        let config = create_test_config();
        let profile = find_profile_directory(&config, "https://unknownsite.com").unwrap();
        assert_eq!(profile, "DefaultProfile");
    }

    #[test]
    fn test_find_profile_no_default_in_config() {
        let mut config = create_test_config();
        config.default_profile = None;
        let profile = find_profile_directory(&config, "https://anotherunknown.org").unwrap();
        assert_eq!(profile, "Default"); // Hardcoded fallback
    }

    #[test]
    fn test_determine_chrome_executable_from_config() {
        let mut config = create_test_config();
        config.chrome_path = Some("/custom/path/to/chrome".to_string());
        assert_eq!(determine_chrome_executable(&config), "/custom/path/to/chrome");
    }

    // Note: Testing the OS-dependent path existence for determine_chrome_executable
    // is harder in unit tests without mocking the filesystem.
    // These tests primarily cover the logic when chrome_path is or isn't in config.
    // For real path detection, integration tests or manual testing on target OS are better.
}
