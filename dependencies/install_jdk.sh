#!/bin/bash

# Oracle JDK 25 installation script for Debian/Ubuntu
# Run with sudo: sudo ./install-oracle-jdk.sh

set -e  # Exit on error

# Configuration
JDK_URL="https://download.oracle.com/java/25/latest/jdk-25_linux-x64_bin.deb"
DOWNLOAD_DIR="/tmp"
INSTALL_DIR="/usr/lib/jvm"
DEB_FILE="$DOWNLOAD_DIR/jdk-25_linux-x64_bin.deb"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}This script must be run as root (use sudo)${NC}" 
   exit 1
fi

echo -e "${GREEN}Starting Oracle JDK 25 installation...${NC}"

# Create installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download JDK
echo -e "${YELLOW}Downloading JDK 25...${NC}"
wget -O "$DEB_FILE" "$JDK_URL" || {
    echo -e "${RED}Failed to download JDK${NC}"
    exit 1
}

# Verify download
if [[ ! -f "$DEB_FILE" ]]; then
    echo -e "${RED}Download failed: $DEB_FILE not found${NC}"
    exit 1
fi

echo -e "${GREEN}Download complete: $DEB_FILE${NC}"

# Extract the .deb file
echo -e "${YELLOW}Extracting package...${NC}"
EXTRACT_DIR="$DOWNLOAD_DIR/jdk-extract"
mkdir -p "$EXTRACT_DIR"
dpkg-deb -x "$DEB_FILE" "$EXTRACT_DIR"

# Find the JDK directory (usually in usr/lib/jvm/oracle-java-25-or something similar)
JDK_SOURCE_DIR=$(find "$EXTRACT_DIR" -type d -path "*/jdk-*" 2>/dev/null | head -n 1)

if [[ -z "$JDK_SOURCE_DIR" ]]; then
    # Alternative: look for the JDK in the extracted structure
    JDK_SOURCE_DIR=$(find "$EXTRACT_DIR" -type d -name "jdk-*" 2>/dev/null | head -n 1)
fi

if [[ -z "$JDK_SOURCE_DIR" ]]; then
    echo -e "${RED}Could not find JDK directory in extracted files${NC}"
    exit 1
fi

echo -e "${GREEN}Found JDK at: $JDK_SOURCE_DIR${NC}"

# Get the actual JDK directory name
JDK_DIR_NAME=$(basename "$JDK_SOURCE_DIR")
TARGET_DIR="$INSTALL_DIR/$JDK_DIR_NAME"

# Remove existing installation if present
if [[ -d "$TARGET_DIR" ]]; then
    echo -e "${YELLOW}Removing existing JDK installation at $TARGET_DIR${NC}"
    rm -rf "$TARGET_DIR"
fi

# Move JDK to installation directory
echo -e "${YELLOW}Installing JDK to $TARGET_DIR${NC}"
mv "$JDK_SOURCE_DIR" "$TARGET_DIR"

# Clean up temporary files
echo -e "${YELLOW}Cleaning up temporary files...${NC}"
rm -rf "$EXTRACT_DIR"
rm -f "$DEB_FILE"

# Set up alternatives for java, javac, etc.
echo -e "${YELLOW}Configuring Java alternatives...${NC}"
update-alternatives --install "/usr/bin/java" "java" "$TARGET_DIR/bin/java" 2025
update-alternatives --install "/usr/bin/javac" "javac" "$TARGET_DIR/bin/javac" 2025
update-alternatives --install "/usr/bin/jar" "jar" "$TARGET_DIR/bin/jar" 2025
update-alternatives --install "/usr/bin/javadoc" "javadoc" "$TARGET_DIR/bin/javadoc" 2025
update-alternatives --install "/usr/bin/jshell" "jshell" "$TARGET_DIR/bin/jshell" 2025

# Set this version as the default (optional - comment out if you don't want to force it)
update-alternatives --set "java" "$TARGET_DIR/bin/java"
update-alternatives --set "javac" "$TARGET_DIR/bin/javac"

# Create environment variable setup script
ENV_SCRIPT="/etc/profile.d/jdk.sh"
echo -e "${YELLOW}Creating environment setup script: $ENV_SCRIPT${NC}"
cat > "$ENV_SCRIPT" << EOF
# Oracle JDK environment variables
export JAVA_HOME="$TARGET_DIR"
export PATH="\$JAVA_HOME/bin:\$PATH"
EOF

chmod 644 "$ENV_SCRIPT"

# Verify installation
echo -e "${GREEN}Installation complete!${NC}"
echo -e "${YELLOW}Verifying installation...${NC}"

# Source the environment script to update current session
source "$ENV_SCRIPT" 2>/dev/null || true

# Check Java version
if command -v java &> /dev/null; then
    JAVA_VERSION=$(java -version 2>&1 | head -n 1)
    echo -e "${GREEN}$JAVA_VERSION${NC}"
else
    echo -e "${RED}Java command not found. You may need to log out and back in.${NC}"
fi

echo
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Oracle JDK 25 installation successful!${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo -e "Installation location: ${YELLOW}$TARGET_DIR${NC}"
echo -e "Java home: ${YELLOW}$TARGET_DIR${NC}"
echo
echo -e "${YELLOW}To use immediately in this terminal, run:${NC}"
echo -e "  source $ENV_SCRIPT"
echo
echo -e "${YELLOW}Or log out and log back in for changes to take effect.${NC}"
echo
echo -e "${YELLOW}To verify installation, run:${NC}"
echo -e "  java -version"
echo -e "  echo \$JAVA_HOME"