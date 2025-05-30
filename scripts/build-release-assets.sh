#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

RELEASE_VERSION=$1 # The version number is passed as an argument

if [ -z "$RELEASE_VERSION" ]; then
  echo "Error: Release version not provided."
  exit 1
fi

echo "Building release assets for version $RELEASE_VERSION..."

ASSET_DIR="release_assets"
rm -rf "$ASSET_DIR" # Clean up previous assets
mkdir -p "$ASSET_DIR"

echo "Cleaning any previous build artifacts..."
rm -rf debian/.debhelper/ debian/files debian/*.log debian/*.substvars debian/profile-switcher/ debian/debhelper-build-stamp

# Build for Linux x86_64 only
TARGET="x86_64-unknown-linux-gnu"
echo "Building for $TARGET..."
cargo build --release --target "$TARGET"

# Build .deb package
echo "Building .deb package..."
./scripts/build-deb-package.sh "$RELEASE_VERSION"

BINARY_NAME="profile_switcher"
# Construct asset name including version and target
OUTPUT_NAME_BASE="profile_switcher-${RELEASE_VERSION}"

cp "target/$TARGET/release/${BINARY_NAME}" "${ASSET_DIR}/${OUTPUT_NAME_BASE}-${TARGET}"

# Copy other assets
echo "Copying additional assets..."
cp profile_switcher.desktop "${ASSET_DIR}/profile_switcher.desktop"
cp LICENSE.md "${ASSET_DIR}/LICENSE.md"
cp README.md "${ASSET_DIR}/README.md"

echo "Release assets prepared in $ASSET_DIR directory:"
ls -l "$ASSET_DIR"
