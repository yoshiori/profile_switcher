#!/bin/bash
# Setup utility for Profile Switcher release assets
set -e

echo "Note: APT repository feature has been removed."
echo "Creating release asset directory..."

# Ensure release_assets directory exists
mkdir -p release_assets

# If there are DEB packages in the current directory, move them to release_assets
if [ -f "*.deb" ]; then
  mv *.deb release_assets/ 2>/dev/null || true
fi

echo "The release will now use direct package downloads instead of an APT repository."
echo "Use 'scripts/install.sh' for installation."

