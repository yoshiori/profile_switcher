#!/bin/bash
# Setup an APT repository for Profile Switcher using GitHub Releases
set -e

# Configuration
REPO_NAME="profile_switcher"
REPO_OWNER="yoshiori"
DIST_NAME="stable"
COMPONENT_NAME="main"
ARCH="amd64"
WORK_DIR="apt-repo"
GPG_KEY_NAME="Profile Switcher Release Key"
GPG_KEY_EMAIL="your-email@example.com" # Replace with your email

# Create directories for APT repo structure
mkdir -p "$WORK_DIR/conf"
mkdir -p "$WORK_DIR/dists/$DIST_NAME/$COMPONENT_NAME/binary-$ARCH"
mkdir -p "$WORK_DIR/pool/$COMPONENT_NAME/$REPO_NAME"

# Create repository configuration
cat > "$WORK_DIR/conf/distributions" << EOF
Origin: $REPO_OWNER
Label: $REPO_NAME
Codename: $DIST_NAME
Architectures: $ARCH
Components: $COMPONENT_NAME
Description: $REPO_NAME APT repository
SignWith: default
EOF

echo "Repository configuration created"

# Copy DEB packages to pool directory
cp release_assets/*.deb "$WORK_DIR/pool/$COMPONENT_NAME/$REPO_NAME/"
echo "DEB packages copied to repository pool"

# Check if GPG key exists, if not create one
if ! gpg --list-keys | grep -q "$GPG_KEY_EMAIL"; then
  echo "Creating GPG key for signing packages..."
  cat > /tmp/gpg-gen-key << EOF
Key-Type: RSA
Key-Length: 4096
Name-Real: $GPG_KEY_NAME
Name-Email: $GPG_KEY_EMAIL
Expire-Date: 0
%no-protection
%commit
EOF
  gpg --batch --gen-key /tmp/gpg-gen-key
  rm /tmp/gpg-gen-key
  echo "GPG key created"
else
  echo "GPG key already exists"
fi

# Initialize the repository using reprepro
cd "$WORK_DIR"
reprepro -C "$COMPONENT_NAME" includedeb "$DIST_NAME" pool/"$COMPONENT_NAME"/"$REPO_NAME"/*.deb

echo "Repository initialized and packages included"

# Create installation instructions
cat > README.repository << EOF
# Installing Profile Switcher via APT Repository

To install Profile Switcher from this APT repository:

## 1. Download and add the GPG key:

\`\`\`bash
curl -fsSL https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/latest/public.gpg | sudo apt-key add -
\`\`\`

## 2. Add the repository:

\`\`\`bash
echo "deb https://github.com/$REPO_OWNER/$REPO_NAME/releases/download/latest/ stable main" | sudo tee /etc/apt/sources.list.d/profile-switcher.list
\`\`\`

## 3. Update and install:

\`\`\`bash
sudo apt-get update
sudo apt-get install profile-switcher
\`\`\`
EOF

echo "Repository setup complete"
echo "Note: You will need to upload the contents of the '$WORK_DIR' directory to your GitHub release"
echo "Also export your public GPG key with: gpg --armor --export $GPG_KEY_EMAIL > public.gpg"

