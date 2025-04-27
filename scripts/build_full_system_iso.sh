#!/bin/bash

# Build the full system ISO
echo "Building full system ISO..."

# Compile the kernel
echo "Compiling the kernel..."
cargo build --release --package vaelix_core

# Create the ISO directory structure
echo "Creating ISO directory structure..."
mkdir -p iso/system/boot
mkdir -p iso/system/kernel

# Copy the compiled kernel to the ISO directory
echo "Copying the compiled kernel to the ISO directory..."
cp target/release/libvaelix_core.so iso/system/kernel/

# Copy the custom bootloader to the ISO directory
echo "Copying the custom bootloader to the ISO directory..."
cp src/boot/vaeboot iso/system/boot/

# Create the ISO file
echo "Creating the ISO file..."
mkisofs -o vaelixos.iso -b boot/vaeboot -no-emul-boot -boot-load-size 4 -boot-info-table iso/system/

echo "Full system ISO build completed."
