# VaelixOS

VaelixOS is a modular, secure, and high-performance operating system built with Rust. It leverages the Linux kernel for hardware compatibility, ensuring a robust and reliable foundation. This document provides a comprehensive overview of the project structure and the development process, making it easier for new contributors to understand and get started.

## Project Structure

The project is organized into several key directories, each serving a specific purpose. Below is a detailed explanation of each directory:

- `tool/`: Contains core binaries essential for the operating system's functionality, such as `vxinit`, `vxlogin`, and `vxpkg`. These binaries are crucial for system initialization, user login, and package management.
- `boot/`: Houses the kernel and bootloader files. This directory is critical for the system's boot process, ensuring the kernel and necessary modules are loaded correctly.
- `dev/`: Contains device nodes, which are special files that represent hardware devices. These nodes are essential for the operating system to interact with hardware components.
- `inx/`: Configuration files that define various system settings and parameters. These files are used to configure the system's behavior and ensure it operates as intended.
- `sys/`: User home directories, where user-specific data and settings are stored. This directory is essential for managing user accounts and their associated data.
- `lib/`: Shared libraries that provide common functionality used across the system. These libraries help in reducing code duplication and ensuring consistency.
- `modules/`: Kernel and driver modules that extend the kernel's functionality. These modules are loaded dynamically, allowing the system to support a wide range of hardware and features.
- `opt/`: Optional software that can be installed and used as needed. This directory is for software that is not essential for the system's core functionality but can enhance its capabilities.
- `tmp/`: Temporary files that are used for short-term storage. These files are typically deleted when they are no longer needed, helping to keep the system clean and efficient.
- `zsr/`: Userland software that runs in user space, separate from the kernel. This software provides various functionalities and services to the user.
- `vix/`: Variable data, such as logs, databases, and cache. This directory is used to store data that changes frequently and is essential for the system's operation.

## Development Process

The development process for VaelixOS involves several key steps:

1. **Kernel Integration**: Use Linux kernel 6.14.4 as the base for hardware support. This ensures that the operating system can interact with a wide range of hardware components.
2. **Bootloader Development**: Implement a custom bootloader (`VaelixBoot`) to load the kernel and modules. This bootloader is crucial for the system's boot process, ensuring that the kernel and necessary modules are loaded correctly.
3. **Subsystems**: Develop modular subsystems for graphics, networking, and user interface. These subsystems provide essential functionalities and can be updated or replaced as needed.
4. **Package Management**: Create a lightweight package manager (`vxp`) for software installation and updates. This package manager ensures that software can be easily installed, updated, and managed.
5. **Documentation**: Provide detailed comments and documentation for all components. This documentation is essential for understanding the system's architecture and ensuring that new contributors can get up to speed quickly.

## Git LFS

Git LFS (Large File Storage) is configured to track large files (e.g., `.xz`, `.tar`, `.iso`) to ensure efficient version control. This configuration helps in managing large files without bloating the repository.

## Getting Started

To get started with VaelixOS, follow these steps:

1. Clone the repository.
2. Install Git LFS: `git lfs install`.
3. Build the project using the provided Makefile or build scripts.

## Contributing

Contributions are welcome! Please follow the coding guidelines and ensure all changes are well-documented. This will help in maintaining the quality and consistency of the codebase.

## Project Structure
- `tool/`: Core binaries (e.g., vxinit, vxlogin, vxpkg).
- `boot/`: Kernel and bootloader files.
- `dev/`: Device nodes.
- `inx/`: Configuration files.
- `sys/`: User home directories.
- `lib/`: Shared libraries.
- `modules/`: Kernel and driver modules.
- `opt/`: Optional software.
- `tmp/`: Temporary files.
- `zsr/`: Userland software.
- `vix/`: Variable data (e.g., logs, databases, cache).

## Development Process
1. **Kernel Integration**: Use Linux kernel 6.14.4 as the base for hardware support.
2. **Bootloader Development**: Implement a custom bootloader (`VaelixBoot`) to load the kernel and modules.
3. **Subsystems**: Develop modular subsystems for graphics, networking, and user interface.
4. **Package Management**: Create a lightweight package manager (`vxp`) for software installation and updates.
5. **Documentation**: Provide detailed comments and documentation for all components.

## Git LFS
Git LFS is configured to track large files (e.g., `.xz`, `.tar`, `.iso`) to ensure efficient version control.

## Getting Started
1. Clone the repository.
2. Install Git LFS: `git lfs install`.
3. Build the project using the provided Makefile or build scripts.

## Contributing
Contributions are welcome! Please follow the coding guidelines and ensure all changes are well-documented.
