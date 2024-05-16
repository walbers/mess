#!/bin/bash

VERSION="0.1.0"
RELEASE_URL="https://github.com/walbers/mess/releases/download/$VERSION"
INSTALL_DIR="/usr/local/bin"

function wsl_notify_install() {
    echo "Installing wsl-send-notify..."
    if ! command -v unzip &> /dev/null; then
        echo "unzip is not installed. Installing unzip..."
        sudo apt-get update
        sudo apt-get install -y unzip
    fi

    curl -fsSLO https://github.com/stuartleeks/wsl-notify-send/releases/download/v0.1.871612270/wsl-notify-send_windows_amd64.zip
    mkdir wsl-notify-send
    unzip wsl-notify-send_windows_amd64.zip -d wsl-notify-send
    sudo cp wsl-notify-send/wsl-notify-send.exe /usr/local/bin/notify-send
    rm -rf wsl-notify-send
    rm wsl-notify-send_windows_amd64.zip
}

function download_and_install() {
    local tar_file="$1.tar.gz"
    local mess_file="$1/mess"
    local config_file="$1/mess.config"
    echo "Downloading $tar_file..."
    curl -L "$RELEASE_URL/$tar_file" -o "$tar_file"
    if [ $? -ne 0 ]; then
        echo "Failed to download $RELEASE_URL/$tar_file"
        exit 1
    fi

    echo "Extracting $tar_file..."
    tar -xzf "$tar_file"

    if [ $? -ne 0 ]; then
        echo "Failed to extract $tar_file"
        exit 1
    fi

    echo "Setting up files"
    chmod +x "$mess_file"
    sudo mv "$mess_file" "$INSTALL_DIR/"
    mkdir ~/.mess
    mv "$config_file" ~/.mess/config

    if [[ "$KERNEL" == *"WSL"* && "$ARCH" == "x86_64" ]]; then
        read -p "Do you want to install wsl-notify-send to receive windows desktop notifcations from WSL? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            wsl_notify_install
        fi
    fi

    rm "$tar_file"
    echo "Configure your ~/.mess/config file before using"
}

OS="$(uname -s)"
ARCH="$(uname -m)"
KERNEL="$(uname -r)"
case "$OS" in
    Linux*)
        MESS_VERSION="mess-$ARCH-unknown-linux-gnu-$VERSION"
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

download_and_install "$MESS_VERSION"
