# VaelixOS Installer

## Overview
The VaelixOS Installer is a script designed to set up the VaelixOS system on a target device. This script handles partitioning, formatting, copying system files, and installing the bootloader.

## Usage
To use the installer, follow these steps:

1. **Prepare the Installation Media**: Create a bootable USB drive with the VaelixOS installer ISO.
2. **Boot from the Installation Media**: Insert the USB drive into the target device and boot from it.
3. **Run the Installer Script**:
   ```bash
   sudo ./installer.sh /dev/sdX
   ```
   Replace `/dev/sdX` with the appropriate device identifier for the target device.

## Important Notes
- **Data Loss Warning**: Running this script will erase all data on the target device. Ensure you have backed up any important data before proceeding.
- **Root Privileges**: The script must be run with root privileges.
- **Target Device**: Specify the correct target device to avoid data loss on other drives.

## Script Details
The installer script performs the following steps:
1. **Partitioning**: Creates a new DOS partition table and partitions the target device.
2. **Formatting**: Formats the partitions with FAT32 for the EFI system partition and ext4 for the root filesystem.
3. **Mounting**: Mounts the partitions to `/mnt` and `/mnt/boot`.
4. **Copying System Files**: Copies the VaelixOS system files to the target device.
5. **Installing Bootloader**: Installs the GRUB bootloader on the target device.
6. **Unmounting**: Unmounts the partitions.

## Customization
- **System Files Path**: Modify the path in the script to point to the location of the VaelixOS system files.
- **Bootloader Configuration**: Adjust the bootloader installation command if using a different bootloader.

## Troubleshooting
- **Partitioning Issues**: Ensure the target device is not in use and is correctly identified.
- **Filesystem Issues**: Verify that the partitions are formatted correctly.
- **Bootloader Issues**: Check the bootloader configuration and ensure it is installed correctly.

## License
This installer script is licensed under the MIT License. See the `LICENSE` file for more details.
