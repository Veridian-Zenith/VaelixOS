# VaelixOS

## Overview
VaelixOS is a next-generation modular operating system featuring a Rust-based microkernel (VaelixCore), a custom windowing system (VegaGX), and a security-first networking stack (VXNet). The goal is to eliminate technical debt from legacy OS structures while ensuring full customization, modern security, and bleeding-edge performance.

## Features
- **Microkernel Architecture**: VaelixCore handles essential services like memory management, IPC, and basic scheduling.
- **Custom Windowing System**: VegaGX provides a high-performance graphical environment with hardware acceleration support.
- **Security-First Networking**: VXNet includes IPv6, WireGuard, and DNSSEC support, with a built-in firewall (VXWall).
- **Package Management**: VXP is designed for performance and modular builds, with a GitHub-based dependency solver.
- **Modular UI Framework**: VXUI is built for customization and modularity, with zero-overhead idle rendering.

## Installation
To install VaelixOS, follow these steps:

1. **Download the Installer ISO**: Download the VaelixOS installer ISO from the [releases page](https://github.com/veridian-zenith/vaelixos/releases).
2. **Create a Bootable USB Drive**: Use a tool like `dd` or `Rufus` to create a bootable USB drive from the ISO file.
3. **Boot from the USB Drive**: Insert the USB drive into the target device and boot from it.
4. **Run the Installer Script**: Follow the on-screen instructions to install VaelixOS on the target device.

## Testing
To test VaelixOS, you can use a virtual machine like QEMU or VirtualBox. Follow these steps to set up a virtual machine:

1. **Download the System ISO**: Download the VaelixOS system ISO from the [releases page](https://github.com/veridian-zenith/vaelixos/releases).
2. **Create a New Virtual Machine**: Create a new virtual machine in QEMU or VirtualBox.
3. **Attach the ISO File**: Attach the VaelixOS system ISO as the bootable disk for the virtual machine.
4. **Configure the Virtual Machine**: Set the appropriate hardware configuration, such as CPU, memory, and storage.
5. **Start the Virtual Machine**: Start the virtual machine and follow the on-screen instructions to install and test VaelixOS.

## Documentation
For more detailed information, please refer to the following documentation:

- [VaelixCore Kernel Documentation](docs/kernel.md)
- [VegaGX Graphics System Documentation](docs/graphics.md)
- [VXNet Networking Stack Documentation](docs/networking.md)
- [VXP Package Management Documentation](docs/package.md)
- [VXUI User Interface Documentation](docs/ui.md)

## License
VaelixOS is licensed under the MIT License. See the `LICENSE` file for more details.
