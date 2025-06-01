#!/bin/bash

set -e

TARGET_DIR="$HOME/.tt"
EXE_PATH="$TARGET_DIR/tt"
URL="https://github.com/dunward/tt/releases/latest/download/tt-macos"

echo "Installing tt from latest release..."
mkdir -p "$TARGET_DIR"
curl -L "$URL" -o "$EXE_PATH"
chmod +x "$EXE_PATH"

if [[ ":$PATH:" != *":$TARGET_DIR:"* ]]; then
    SHELL_NAME=$(basename "$SHELL")
    
    if [[ "$SHELL_NAME" == "zsh" ]]; then
        PROFILE_FILE="$HOME/.zshrc"
    elif [[ "$SHELL_NAME" == "bash" ]]; then
        PROFILE_FILE="$HOME/.bashrc"
    else
        PROFILE_FILE="$HOME/.profile"
    fi

    echo "export PATH=\"\$PATH:$TARGET_DIR\"" >> "$PROFILE_FILE"
    echo "Added $TARGET_DIR to PATH in $PROFILE_FILE (restart your terminal or run 'source $PROFILE_FILE')"
else
    echo "$TARGET_DIR is already in your PATH."
fi

echo -e "\n tt installation complete! You can now run 'tt' from your terminal."
