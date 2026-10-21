// Copyright (c) 2025 Yoshiori Shoji
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::{anyhow, Context, Result};
use std::path::Path;
use url::Url;

/// Returns `arg` as a URL string, converting local file paths into `file://` URLs.
///
/// Even with `%U` in the desktop entry, GIO (Files app, `gio open`) passes
/// local files as plain paths instead of `file://` URIs.
pub fn normalize_target(arg: &str) -> Result<String> {
    if Url::parse(arg).is_ok() {
        return Ok(arg.to_string());
    }

    let absolute_path = std::path::absolute(Path::new(arg))
        .with_context(|| format!("Failed to resolve file path: {}", arg))?;
    Url::from_file_path(&absolute_path)
        .map(String::from)
        .map_err(|_| {
            anyhow!(
                "'{}' is neither a valid URL nor a local file path. Pass a URL like https://example.com or an existing file path.",
                arg
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keeps_https_url_unchanged() {
        let url = "https://example.com/path?q=1";
        assert_eq!(normalize_target(url).unwrap(), url);
    }

    #[test]
    fn test_keeps_file_url_unchanged() {
        let url = "file:///home/user/page.html";
        assert_eq!(normalize_target(url).unwrap(), url);
    }

    #[test]
    fn test_converts_absolute_path_to_file_url() {
        assert_eq!(
            normalize_target("/home/user/page.html").unwrap(),
            "file:///home/user/page.html"
        );
    }

    #[test]
    fn test_percent_encodes_special_characters_in_path() {
        assert_eq!(
            normalize_target("/home/user/my page#1.html").unwrap(),
            "file:///home/user/my%20page%231.html"
        );
    }

    #[test]
    fn test_converts_relative_path_to_absolute_file_url() {
        // Avoid comparing against current_dir(): other tests change the CWD concurrently.
        let normalized = normalize_target("page.html").unwrap();
        assert!(normalized.starts_with("file:///"), "got {}", normalized);
        assert!(normalized.ends_with("/page.html"), "got {}", normalized);
    }
}
