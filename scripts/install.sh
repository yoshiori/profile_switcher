#!/bin/bash
# Install Profile Switcher from GitHub Releases
set -e

REPO_OWNER="yoshiori"
REPO_NAME="profile_switcher"
LATEST_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"

echo "Installing Profile Switcher..."

# Check if required tools are available
command -v curl >/dev/null 2>&1 || { echo "Error: curl is required but not installed."; exit 1; }
command -v apt-get >/dev/null 2>&1 || { echo "Error: apt-get is required but not installed. This script only supports Debian-based systems."; exit 1; }
command -v sudo >/dev/null 2>&1 || { echo "Error: sudo is required but not installed."; exit 1; }

# Get latest release information
echo "Fetching the latest release information..."
RELEASE_INFO=$(curl -s "$LATEST_URL")
TAG_NAME=$(echo "$RELEASE_INFO" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)

if [ -z "$TAG_NAME" ]; then
  echo "Error: Could not determine latest version."
  exit 1
fi

echo "Latest version: $TAG_NAME"

# Download the DEB package
DEB_URL="https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/$TAG_NAME/profile-switcher_${TAG_NAME#v}_amd64.deb"
DEB_FILE="/tmp/profile-switcher_${TAG_NAME#v}_amd64.deb"

echo "Downloading package from: $DEB_URL"
curl -L "$DEB_URL" -o "$DEB_FILE"

if [ ! -f "$DEB_FILE" ]; then
  echo "Error: Failed to download the DEB package."
  exit 1
fi

# Install the DEB package
echo "Installing DEB package..."
sudo apt-get install -y "$DEB_FILE"

# Clean up
rm "$DEB_FILE"

echo "Profile Switcher has been installed successfully!"
echo "You can now launch it by running 'profile_switcher' or using the application icon in your desktop environment."

