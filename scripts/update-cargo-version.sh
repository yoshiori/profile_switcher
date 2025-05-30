#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

# This script updates the version in Cargo.toml and Cargo.lock

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Error: Version number not provided."
  exit 1
fi

echo "Updating Cargo.toml to version $VERSION..."

# Remove 'v' prefix if present
VERSION="${VERSION#v}"

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update Cargo.lock by running cargo update
cargo update --package profile_switcher

echo "Version updated to $VERSION in Cargo.toml and Cargo.lock"
