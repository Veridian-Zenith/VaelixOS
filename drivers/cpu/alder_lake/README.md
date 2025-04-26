# Linux Driver Source for cpu/alder_lake

Extracted from Linux 6.14.4 for VaelixOS hardware support.
These files serve as reference for implementing Rust shims.

## Purpose
This directory contains the Linux driver code that will be used as reference
for implementing the corresponding Rust hardware abstraction layer in VaelixOS.

## Integration Process
1. Study the driver implementation
2. Identify key hardware interaction points
3. Create corresponding Rust interfaces
4. Implement minimal shims for required functionality

## Hardware Details
Intel Core i3-1215U (Alder Lake)\n- 6 cores (2 P-cores, 4 E-cores)\n- Support for AVX, AVX2, SSE4.2
