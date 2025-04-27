#!/bin/bash

# Build the entire project
echo "Building the entire project..."

# Compile the kernel
echo "Compiling the kernel..."
cargo build --release --package vaelix_core

# Compile the graphics modules
echo "Compiling the graphics modules..."
cargo build --release --package vaelix_graphics

# Compile the networking modules
echo "Compiling the networking modules..."
cargo build --release --package vaelix_networking

# Compile the package management modules
echo "Compiling the package management modules..."
cargo build --release --package vaelix_package

# Compile the UI modules
echo "Compiling the UI modules..."
cargo build --release --package vaelix_ui

echo "Build completed."
