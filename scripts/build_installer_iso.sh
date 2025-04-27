#!/bin/bash

# Build the installer ISO
echo "Building installer ISO..."

# Compile the kernel
echo "Compiling the kernel..."
cargo build --release --package vaelix_core

# Create the ISO directory structure
echo "Creating ISO directory structure..."
mkdir -p iso/installer/boot
mkdir -p iso/installer/kernel

# Copy the compiled kernel to the ISO directory
echo "Copying the compiled kernel to the ISO directory..."
cp target/release/libvaelix_core.so iso/installer/kernel/

# Copy the custom bootloader to the ISO directory
echo "Copying the custom bootloader to the ISO directory..."
cp src/boot/vaeboot iso/installer/boot/

# Copy the installer script to the ISO directory
echo "Copying the installer script to the ISO directory..."
cp iso/installer/installer.sh iso/installer/

# Create the ISO file
echo "Creating the ISO file..."
mkisofs -o installer.iso -b boot/vaeboot -no-emul-boot -boot-load-size 4 -boot-info-table iso/installer/

echo "Installer ISO build completed."
