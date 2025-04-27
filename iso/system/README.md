# VaelixOS System ISO

## Overview
The VaelixOS System ISO contains the full VaelixOS operating system, including the kernel, graphics system, networking stack, package management, and user interface components. This ISO is designed to be used for testing and development purposes, allowing you to run VaelixOS in a virtual machine or on physical hardware.

## Usage
To use the VaelixOS System ISO, follow these steps:

1. **Create a Bootable USB Drive**: Use a tool like `dd` or `Rufus` to create a bootable USB drive from the ISO file.
2. **Boot from the USB Drive**: Insert the USB drive into the target device and boot from it.
3. **Install VaelixOS**: Follow the on-screen instructions to install VaelixOS on the target device.

## System Components
The VaelixOS System ISO includes the following components:

- **Kernel (VaelixCore)**: The microkernel responsible for memory management, IPC, and basic scheduling.
- **Graphics System (VegaGX)**: The custom windowing system with hardware acceleration support.
- **Networking Stack (VXNet)**: The security-first networking stack with IPv6, WireGuard, and DNSSEC support.
- **Package Management (VXP)**: The package resolution system designed for performance and modular builds.
- **User Interface (VXUI)**: The Rust-based GUI framework built for modularity and customization.

## Customization
The VaelixOS System ISO is fully customizable. You can modify the system components, add new features, and customize the user interface to suit your needs.

## Testing
To test the VaelixOS System ISO, you can use a virtual machine like QEMU or VirtualBox. Follow these steps to set up a virtual machine:

1. **Create a New Virtual Machine**: Create a new virtual machine in QEMU or VirtualBox.
2. **Attach the ISO File**: Attach the VaelixOS System ISO as the bootable disk for the virtual machine.
3. **Configure the Virtual Machine**: Set the appropriate hardware configuration, such as CPU, memory, and storage.
4. **Start the Virtual Machine**: Start the virtual machine and follow the on-screen instructions to install and test VaelixOS.

## License
This system ISO is licensed under the MIT License. See the `LICENSE` file for more details.
