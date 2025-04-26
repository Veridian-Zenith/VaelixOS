#!/bin/bash
# Driver extraction script for VaelixOS
# This script extracts relevant driver code from Linux 6.14.4

LINUX_SRC="/home/dae/Veridian-Zenith/VaelixOS/linux-6.14.4"
DRIVERS_BASE="/home/dae/Veridian-Zenith/VaelixOS/drivers"

# Array of drivers to extract
declare -A DRIVERS=(
    # CPU support (Alder Lake)
    ["cpu/alder_lake"]="arch/x86/kernel/cpu/intel* arch/x86/kernel/cpu/hybrid* arch/x86/include/asm/intel*"

    # GPU support (Intel UHD)
    ["gpu/intel"]="drivers/gpu/drm/i915/* include/drm/i915*"

    # Network support
    ["net/wireless"]="drivers/net/wireless/realtek/rtw89/* drivers/net/wireless/realtek/rtw89/rtw8852b* include/net/mac80211.h"
    ["net/ethernet"]="drivers/net/ethernet/realtek/r8169* include/linux/netdevice.h"

    # Storage support (NVMe)
    ["storage/nvme"]="drivers/nvme/* include/linux/nvme*"

    # Audio support
    ["audio/hda"]="sound/hda/* sound/soc/intel/sof/* include/sound/hda*"

    # Bluetooth support
    ["bluetooth/rtk"]="drivers/bluetooth/btusb* drivers/bluetooth/rtk_* include/net/bluetooth/*"

    # Base support
    ["base/common"]="include/linux/pci.h include/linux/interrupt.h include/linux/dma-mapping.h"
)

# Clean existing extracted files
for dir in "${!DRIVERS[@]}"; do
    rm -rf "$DRIVERS_BASE/$dir"/*
done

# Create directories and extract drivers
for dir in "${!DRIVERS[@]}"; do
    mkdir -p "$DRIVERS_BASE/$dir"

    # Extract files
    for pattern in ${DRIVERS[$dir]}; do
        echo "Extracting $pattern to $dir..."
        cd "$LINUX_SRC" && find . -path "*/$pattern" -type f \( -name "*.c" -o -name "*.h" \) -exec cp --parents {} "$DRIVERS_BASE/$dir/" \;
    done

    # Create README for each driver
    cat > "$DRIVERS_BASE/$dir/README.md" << EOL
# Linux Driver Source for $dir

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
$(case $dir in
    "cpu/alder_lake") echo "Intel Core i3-1215U (Alder Lake)\n- 6 cores (2 P-cores, 4 E-cores)\n- Support for AVX, AVX2, SSE4.2" ;;
    "gpu/intel") echo "Intel UHD Graphics (Alder Lake-UP3 GT1)\n- Device ID: 8086:46b3\n- OpenGL 4.6, Vulkan 1.4.309" ;;
    "net/wireless") echo "Realtek RTL8852BE PCIe 802.11ax\n- 2.5 GT/s PCIe\n- 802.11ax support" ;;
    "net/ethernet") echo "Realtek RTL8111/8168 Gigabit Ethernet\n- 2.5 GT/s PCIe" ;;
    "storage/nvme") echo "KIOXIA NVMe SSD\n- PCIe Gen3 x4 (63.2 Gb/s)\n- 238.47 GiB capacity" ;;
    "audio/hda") echo "Intel Alder Lake PCH-P HD Audio\n- Device ID: 8086:51c8\n- SOF audio support" ;;
    "bluetooth/rtk") echo "Realtek Bluetooth Radio\n- USB interface (12 Mb/s)\n- Bluetooth 5.2 support" ;;
    *) echo "Common hardware interfaces and utilities" ;;
esac)
EOL
done

echo "Driver extraction complete. Check individual directories for extracted source files."
