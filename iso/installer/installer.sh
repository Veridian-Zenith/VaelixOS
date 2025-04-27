#!/bin/bash

# VaelixOS Installer Script
# This script sets up the VaelixOS system on a target device.

# Function to print usage information
usage() {
    echo "Usage: $0 /dev/sdX"
    echo "  /dev/sdX: Target device to install VaelixOS on"
    exit 1
}

# Check if the script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

# Check if the target device is provided
if [ -z "$1" ]; then
    usage
fi

TARGET_DEVICE=$1

# Confirm the target device
echo "WARNING: This will erase all data on $TARGET_DEVICE"
read -p "Are you sure you want to continue? (y/n): " confirm
if [ "$confirm" != "y" ]; then
    echo "Installation aborted."
    exit 1
fi

# Partition the target device
echo "Partitioning $TARGET_DEVICE..."
(
echo o # Create a new empty DOS partition table
echo n # Add a new partition
echo p # Primary partition
echo 1 # Partition number
echo   # First sector (Accept default: 2048)
echo +512M # Last sector (512MB for EFI)
echo t # Change partition type
echo 1 # Partition number
echo 1 # EFI System (FAT32/LBA)
echo n # Add a new partition
echo p # Primary partition
echo 2 # Partition number
echo   # First sector (Accept default)
echo   # Last sector (Accept default: remaining space)
echo w # Write changes
) | fdisk $TARGET_DEVICE

# Format the partitions
echo "Formatting partitions..."
mkfs.vfat ${TARGET_DEVICE}1
mkfs.ext4 ${TARGET_DEVICE}2

# Mount the partitions
echo "Mounting partitions..."
mount ${TARGET_DEVICE}2 /mnt
mkdir /mnt/boot
mount ${TARGET_DEVICE}1 /mnt/boot

# Copy the VaelixOS system files
echo "Copying VaelixOS system files..."
cp -r /path/to/vaelixos/system/* /mnt/

# Install the bootloader
echo "Installing bootloader..."
grub-install --target=i386-pc --recheck $TARGET_DEVICE
grub-mkconfig -o /mnt/boot/grub/grub.cfg

# Unmount the partitions
echo "Unmounting partitions..."
umount /mnt/boot
umount /mnt

echo "VaelixOS installation complete."
