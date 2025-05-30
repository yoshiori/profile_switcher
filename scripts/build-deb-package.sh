#!/bin/bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Error: Version not provided."
  exit 1
fi

echo "Building .deb package for version $VERSION..."

# Generate changelog from template
DATE=$(date -R)
sed -e "s/\${VERSION}/$VERSION/g" -e "s/\${DATE}/$DATE/g" debian/changelog.template > debian/changelog

# Build the Debian package
dpkg-buildpackage -b -us -uc

# Move the built package to the release_assets directory
mkdir -p release_assets
mv ../profile-switcher_${VERSION}_amd64.deb release_assets/

echo "Debian package created: release_assets/profile-switcher_${VERSION}_amd64.deb"
