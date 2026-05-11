#!/bin/bash
# Installation script for roadmap-cli
# Usage: curl -fsSL https://raw.githubusercontent.com/Siovos/roadmap-cli/main/install.sh | bash

set -e

REPO="Siovos/roadmap-cli"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  darwin)
    case "$ARCH" in
      x86_64) BINARY="roadmap-cli-darwin-x86_64" ;;
      arm64)  BINARY="roadmap-cli-darwin-arm64" ;;
      *)      echo "Architecture non supportee: $ARCH"; exit 1 ;;
    esac
    ;;
  linux)
    case "$ARCH" in
      x86_64) BINARY="roadmap-cli-linux-x86_64" ;;
      aarch64) BINARY="roadmap-cli-linux-arm64" ;;
      *)      echo "Architecture non supportee: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "OS non supporte: $OS"
    exit 1
    ;;
esac

# Get latest release
echo "Telechargement de roadmap-cli..."
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)

if [ -z "$LATEST" ]; then
  echo "Impossible de recuperer la derniere version"
  echo "Verifiez que le repo $REPO a des releases sur GitHub"
  exit 1
fi

echo "Version: $LATEST"

URL="https://github.com/$REPO/releases/download/$LATEST/$BINARY.tar.gz"

# Download and install
TMP=$(mktemp -d)
curl -fsSL "$URL" -o "$TMP/roadmap-cli.tar.gz"
tar -xzf "$TMP/roadmap-cli.tar.gz" -C "$TMP"

echo "Installation dans $INSTALL_DIR..."

# Install both roadmap-cli and roadmap symlink
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP/roadmap-cli" "$INSTALL_DIR/roadmap-cli"
  chmod +x "$INSTALL_DIR/roadmap-cli"
  ln -sf "$INSTALL_DIR/roadmap-cli" "$INSTALL_DIR/roadmap"
else
  sudo mv "$TMP/roadmap-cli" "$INSTALL_DIR/roadmap-cli"
  sudo chmod +x "$INSTALL_DIR/roadmap-cli"
  sudo ln -sf "$INSTALL_DIR/roadmap-cli" "$INSTALL_DIR/roadmap"
fi

# Cleanup
rm -rf "$TMP"

echo ""
echo "roadmap-cli $LATEST installe avec succes!"
echo ""
echo "  roadmap init          # Initialiser dans un projet"
echo "  roadmap --help        # Voir toutes les commandes"
echo "  roadmap report        # Rapport de progression"
