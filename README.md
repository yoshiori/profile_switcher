# Profile Switcher

Profile Switcher is a command-line tool written in Rust that launches Google Chrome with a specific profile based on the URL provided as an argument. The mapping between URLs and Chrome profiles is defined in a `config.toml` file.

## Features

*   Opens Chrome with a specific profile based on URL patterns.
*   Configuration via a simple TOML file.
*   Automatically detects Chrome's executable path (can be overridden in config).
*   Supports a default profile for URLs that don't match any pattern.

## Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/yoshiori/profile_switcher.git
    cd profile_switcher
    ```
2.  Build the project:
    ```bash
    cargo build
    ```
    For a release build, use:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/debug/profile_switcher` or `target/release/profile_switcher`.

## Configuration

Create a `config.toml` file. The application will look for this file in the following locations, in order:
1.  Next to the executable.
2.  In the project root directory (useful for development).

If the configuration file is not found, the program will print an example configuration to the console and exit.

Here's an example `config.toml`:

```toml
# Optional: Specify the default profile to use if no URL pattern matches.
# If not specified, Chrome's actual default profile ("Default") will be used.
default_profile = "DefaultProfileName"

# Optional: Specify the full path to the Chrome executable.
# If not specified, the program will try to find it in common locations.
# chrome_path = "/usr/bin/google-chrome-stable" # Example for Linux
# chrome_path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" # Example for macOS

[[profiles]]
name = "Work"
# This is the directory name of your Chrome profile (e.g., "Profile 1", "Default")
# You can find these in your Chrome user data directory.
# Linux: ~/.config/google-chrome/
# macOS: ~/Library/Application Support/Google/Chrome/
# Windows: %LOCALAPPDATA%\Google\Chrome\User Data\
profile_directory = "Profile 1"
# List of strings to match against the URL's host or the full URL.
# The longest matching pattern determines the profile.
url_patterns = ["github.com", "work.example.com", "jira.mycompany.net"]

[[profiles]]
name = "Personal"
profile_directory = "Profile 2"
url_patterns = ["youtube.com", "personal.blog.com"]

[[profiles]]
name = "Shopping"
profile_directory = "Profile 3"
url_patterns = ["amazon.com", "ebay.com"]

# Add more profile mappings as needed.
```

**Finding Profile Directory Names:**

*   **Chrome:** Type `chrome://version` in your Chrome address bar and look for the "Profile Path". The last component of this path is usually your profile directory name (e.g., `Default`, `Profile 1`, `Profile 2`).

## Usage

Run the program from your terminal, providing a URL as an argument:

```bash
./target/debug/profile_switcher "https://github.com"
```

Or, if you built for release:

```bash
./target/release/profile_switcher "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

The program will then open the given URL in Chrome using the profile associated with the URL's domain or the full URL string, as defined in your `config.toml`.

## Future Enhancements (Planned)

*   Integration with the operating system to be selectable as a default browser.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
