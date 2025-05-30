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

## Installation

### Option 1: Installing with Quick Install Script (Ubuntu/Debian)

The quickest way to install Profile Switcher on Ubuntu or Debian-based distributions:

```bash
curl -fsSL https://raw.githubusercontent.com/yoshiori/profile_switcher/main/scripts/install.sh | bash
```

This script will:
- Download the latest release from GitHub
- Install the package automatically using apt
- Set up the application so it's ready to use

### Option 2: Using APT Repository (Ubuntu/Debian)

For a more integrated installation experience, you can use our APT repository:

1. **Add the repository GPG key**:
   ```bash
   curl -fsSL https://github.com/yoshiori/profile_switcher/releases/download/latest/public.gpg | sudo apt-key add -
   ```

2. **Add the repository to your sources**:
   ```bash
   echo "deb https://github.com/yoshiori/profile_switcher/releases/download/latest/ stable main" | sudo tee /etc/apt/sources.list.d/profile-switcher.list
   ```

3. **Update and install**:
   ```bash
   sudo apt-get update
   sudo apt-get install profile-switcher
   ```

### Option 3: Manual DEB Package Installation (Ubuntu/Debian)

You can also install Profile Switcher by downloading and installing the DEB package manually:

1.  **Download the latest .deb package** from the [GitHub Releases page](https://github.com/yoshiori/profile_switcher/releases).

2.  **Install the package**:
    ```bash
    sudo dpkg -i profile-switcher_X.Y.Z_amd64.deb
    # If there are dependencies missing, install them with:
    sudo apt install -f
    ```

    This will:
    - Install the binary to `/usr/bin/profile_switcher`
    - Install the desktop file to `/usr/share/applications/`
    - Install a default configuration template for new user accounts

    Note: When you first run the application, it will attempt to create a default configuration file at `~/.config/profile_switcher/config.toml`.

### Option 4: Manual Installation

To manually install Profile Switcher on Linux, you need to install the `.desktop` file and update the system's MIME type associations.

1.  **Install the `.desktop` file:**

    Copy the `profile_switcher.desktop` file to your local applications directory:
    ```bash
    mkdir -p ~/.local/share/applications/
    cp profile_switcher.desktop ~/.local/share/applications/profile_switcher.desktop
    ```
    Then, update the applications database:
    ```bash
    update-desktop-database ~/.local/share/applications/
    ```

2.  **Set as default browser:**

    You can then set Profile Switcher as the default handler for `http` and `https` schemes using `xdg-settings`:
    ```bash
    xdg-settings set default-web-browser profile_switcher.desktop
    xdg-settings check default-web-browser profile_switcher.desktop
    ```
    The second command should confirm that `profile_switcher.desktop` is indeed the default.

    Alternatively, you might be able to set this through your desktop environment's system settings (e.g., GNOME Control Center, KDE System Settings). Look for "Default Applications" or similar.

3.  **Build and Install `profile_switcher`:**

    Ensure that the `profile_switcher` executable is built and placed in a directory included in your system's `PATH` (e.g., `~/.local/bin/` or `/usr/local/bin/`).
    ```bash
    cargo build --release
    # Example:
    # cp target/release/profile_switcher ~/.local/bin/
    ```

Now, when you click a link in most applications, it should open with Profile Switcher.

## Configuration

Create a `config.toml` file. The application will look for this file in the following locations, in order:
1.  `~/.config/profile_switcher/config.toml` (User-specific configuration)
2.  Next to the executable.
3.  In the project root directory (useful for development).

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
