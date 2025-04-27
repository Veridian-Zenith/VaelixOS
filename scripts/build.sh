#!/bin/bash

# VaelixOS Build Script
# This script compiles the VaelixOS project and generates the installer and system ISOs.

# Set up environment variables
export VAELIXOS_ROOT="/vaelixos"
export VAELIXOS_KERNEL="$VAELIXOS_ROOT/kernel"
export VAELIXOS_GRAPHICS="$VAELIXOS_ROOT/graphics"
export VAELIXOS_UI="$VAELIXOS_ROOT/ui"
export VAELIXOS_NETWORKING="$VAELIXOS_ROOT/networking"
export VAELIXOS_PACKAGE="$VAELIXOS_ROOT/package"
export VAELIXOS_BOOT="$VAELIXOS_ROOT/boot"

# Compile the project
cargo build --release

# Create the installer ISO
mkdir -p iso/installer
cp target/release/vaelixos iso/installer
cp iso/installer/installer.sh iso/installer
genisoimage -o vaelixos-installer.iso -b installer.sh -no-emul-boot -boot-load-size 4 -boot-info-table -J -R iso/installer

# Create the system ISO
mkdir -p iso/system
cp -r $VAELIXOS_ROOT iso/system
genisoimage -o vaelixos-system.iso -J -R iso/system

echo "Build completed successfully."
