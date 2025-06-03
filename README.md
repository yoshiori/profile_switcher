# Profile Switcher

Profile Switcher is a smart browser launcher written in Rust that automatically opens URLs in the appropriate Google Chrome profile. When set as your default browser, it seamlessly redirects web links to the correct Chrome profile based on configurable URL patterns.

## How It Works

When you click a link in any application (email, Slack, terminal, etc.), Profile Switcher:
1. **Intercepts the URL** as your default browser
2. **Analyzes the URL** against your configured patterns
3. **Launches Chrome** with the appropriate profile automatically
4. **Keeps your browsing contexts separate** - work links open in your work profile, personal links in your personal profile

## Key Features

*   **Automatic profile selection** based on URL patterns
*   **Seamless integration** as your system's default browser
*   **Simple TOML configuration** with pattern matching
*   **Chrome executable auto-detection** (with manual override support)
*   **Fallback to default profile** for unmatched URLs
*   **Cross-platform support** (Linux, macOS, Windows)

## Installation

## Installation

### Quick Install (Ubuntu/Debian)

The easiest way to get started on Ubuntu or Debian-based distributions:

```bash
curl -fsSL https://raw.githubusercontent.com/yoshiori/profile_switcher/main/scripts/install.sh | bash
```

This script will:
- Download and install the latest release automatically
- Set up the application and make it available system-wide
- Configure it as a browser option in your system

### Manual Installation Options

#### Option 1: DEB Package (Ubuntu/Debian)

1.  **Download the latest .deb package** from the [GitHub Releases page](https://github.com/yoshiori/profile_switcher/releases).

2.  **Install the package**:
    ```bash
    sudo dpkg -i profile-switcher_X.Y.Z_amd64.deb
    # If there are dependencies missing, install them with:
    sudo apt install -f
    ```

#### Option 2: Manual Setup (All Linux Distributions)

1.  **Build from source:**
    ```bash
    git clone https://github.com/yoshiori/profile_switcher.git
    cd profile_switcher
    cargo build --release
    ```

2.  **Install the executable:**
    ```bash
    # Copy to a directory in your PATH
    cp target/release/profile_switcher ~/.local/bin/
    # or
    sudo cp target/release/profile_switcher /usr/local/bin/
    ```

3.  **Install the desktop file:**
    ```bash
    mkdir -p ~/.local/share/applications/
    cp profile_switcher.desktop ~/.local/share/applications/
    update-desktop-database ~/.local/share/applications/
    ```

### Setting as Default Browser

After installation, set Profile Switcher as your default browser:

```bash
xdg-settings set default-web-browser profile_switcher.desktop
```

Verify it's set correctly:
```bash
xdg-settings check default-web-browser profile_switcher.desktop
```

Alternatively, you can set this through your desktop environment's system settings (GNOME Settings, KDE System Settings, etc.) under "Default Applications."

## Configuration

Profile Switcher uses a `config.toml` file to define which Chrome profiles to use for different URLs. The application looks for this file in the following locations (in order):

1.  `~/.config/profile_switcher/config.toml` (recommended)
2.  Next to the executable
3.  In the project root directory (for development)

If no configuration file is found, the program will display an example configuration and exit.

### Example Configuration

```toml
# Optional: Default profile for URLs that don't match any pattern
# If not specified, Chrome's default profile ("Default") will be used
default_profile = "DefaultProfileName"

# Optional: Custom Chrome executable path
# If not specified, Profile Switcher will auto-detect Chrome
# chrome_path = "/usr/bin/google-chrome-stable"           # Linux
# chrome_path = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"  # macOS

[[profiles]]
name = "Work"
profile_directory = "Profile 1"
url_patterns = ["github.com", "work.example.com", "jira.mycompany.net"]

[[profiles]]
name = "Personal"
profile_directory = "Profile 2"
url_patterns = ["youtube.com", "personal.blog.com"]

[[profiles]]
name = "Shopping"
profile_directory = "Profile 3"
url_patterns = ["amazon.com", "ebay.com"]
```

### Finding Your Chrome Profile Directories

To find your Chrome profile directory names:

1. Open Chrome and go to `chrome://version`
2. Look for the "Profile Path" field
3. The last part of the path is your profile directory name (e.g., `Default`, `Profile 1`, `Profile 2`)

**Profile locations by OS:**
- **Linux:** `~/.config/google-chrome/`
- **macOS:** `~/Library/Application Support/Google/Chrome/`
- **Windows:** `%LOCALAPPDATA%\Google\Chrome\User Data\`

## Usage

### Primary Usage (As Default Browser)

Once installed and configured as your default browser, Profile Switcher works automatically:

1. **Click any link** in emails, Slack, terminal, or other applications
2. **Profile Switcher intercepts the URL** and analyzes it against your patterns
3. **Chrome opens automatically** with the correct profile
4. **Your browsing stays organized** - work links in work profile, personal links in personal profile

### Manual Usage (Command Line)

You can also run Profile Switcher directly from the command line:

```bash
profile_switcher "https://github.com"
```

Or during development:
```bash
./target/release/profile_switcher "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

### Example Workflow

```
You receive an email with a GitHub link
    ↓
Click the link
    ↓
Profile Switcher detects "github.com"
    ↓
Chrome opens in your "Work" profile automatically
    ↓
You're signed in with your work GitHub account!
```

## Common Use Cases

- **Work/Personal Separation**: Automatically open work-related sites (GitHub, company tools) in your work profile, and personal sites (YouTube, social media) in your personal profile
- **Client Management**: Different profiles for different clients or projects
- **Development/Production**: Separate profiles for development and production environments
- **Shopping Accounts**: Dedicated profile for shopping sites with your saved payment methods

## Troubleshooting

### Configuration Issues
- Make sure your `config.toml` file exists in `~/.config/profile_switcher/`
- Verify profile directory names match exactly with Chrome's profile directories
- Check that Chrome is installed and accessible

### Default Browser Not Working
- Verify the desktop file is installed: `ls ~/.local/share/applications/ | grep profile_switcher`
- Check default browser setting: `xdg-settings check default-web-browser profile_switcher.desktop`
- Try setting again: `xdg-settings set default-web-browser profile_switcher.desktop`

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
