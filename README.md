# VaelixOS

VaelixOS is a next-generation modular operating system featuring a Rust-based microkernel (VaelixCore), a custom windowing system (VegaGX), and a security-first networking stack (VXNet). The goal is to eliminate technical debt from legacy OS structures while ensuring full customization, modern security, and bleeding-edge performance.

## Project Structure

- `src/kernel`: Contains the core kernel modules.
- `src/graphics`: Contains the graphics modules.
- `src/networking`: Contains the networking modules.
- `src/package`: Contains the package management modules.
- `src/ui`: Contains the UI modules.
- `src/boot`: Contains the bootloader modules.
- `scripts`: Contains build and test scripts.

## Building the Project

To build the entire project, run the following command:

```bash
./scripts/build.sh
```

## Running Tests

To run the tests for the kernel modules, run the following command:

```bash
./scripts/test.sh
```

## Building the Full System ISO

To build the full system ISO, run the following command:

```bash
./scripts/build_full_system_iso.sh
```

## Building the Installer ISO

To build the installer ISO, run the following command:

```bash
./scripts/build_installer_iso.sh
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
