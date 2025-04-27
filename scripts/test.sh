#!/bin/bash

# Run tests for the kernel modules
echo "Running tests for the kernel modules..."

# Run tests for the vaelix_core package
echo "Running tests for vaelix_core..."
cargo test --package vaelix_core

# Run tests for the vaelix_graphics package
echo "Running tests for vaelix_graphics..."
cargo test --package vaelix_graphics

# Run tests for the vaelix_networking package
echo "Running tests for vaelix_networking..."
cargo test --package vaelix_networking

# Run tests for the vaelix_package package
echo "Running tests for vaelix_package..."
cargo test --package vaelix_package

# Run tests for the vaelix_ui package
echo "Running tests for vaelix_ui..."
cargo test --package vaelix_ui

echo "All tests completed."
