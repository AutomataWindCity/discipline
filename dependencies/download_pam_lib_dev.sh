#!/bin/bash
# Script to download and unpack Linux-PAM repository

set -e  # Exit on error

# Default folder if none specified
TARGET_DIR="/workspaces/discipline/dependencies/linux-pam"

# Repository URL
REPO_URL="https://github.com/linux-pam/linux-pam.git"

echo "Downloading Linux-PAM repository to: $TARGET_DIR"

# Create target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Check if git is available
if ! command -v git &> /dev/null; then
  echo "Error: git is not installed. Installing git..."
  sudo apt-get update
  sudo apt-get install -y git
fi

# Clone the repository
echo "Cloning repository from $REPO_URL..."
git clone "$REPO_URL" "$TARGET_DIR"

# Check if clone was successful
if [ $? -eq 0 ]; then
  echo "Successfully cloned Linux-PAM repository to $TARGET_DIR"
  echo "Repository size:"
  du -sh "$TARGET_DIR"
else
  echo "Error: Failed to clone repository"
  exit 1
fi