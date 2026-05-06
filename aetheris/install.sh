#!/bin/bash
# Install script for Aetheris

echo "[*] Building Aetheris in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "[+] Build successful."
    echo "[*] Installing to /usr/local/bin/aetheris (requires sudo)..."
    sudo cp target/release/aetheris /usr/local/bin/aetheris
    if [ $? -eq 0 ]; then
        echo "[+] Installation complete! You can now run 'aetheris <IP>' from anywhere."
    else
        echo "[-] Installation failed. Do you have sudo privileges?"
    fi
else
    echo "[-] Build failed."
fi
