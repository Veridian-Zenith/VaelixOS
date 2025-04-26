#!/bin/bash
# Firmware extraction script for VaelixOS
# This script extracts firmware blobs needed by our drivers

LINUX_SRC="../linux-6.14.4"
FIRMWARE_DIR="./firmware"

# Create firmware directory structure
mkdir -p "$FIRMWARE_DIR"/{gpu,wifi,net,bluetooth}

# Array of firmware files to extract
declare -A FIRMWARE=(
    # GPU firmware (Intel i915)
    ["gpu/i915"]="drivers/gpu/drm/i915/firmware/adlp_dmc.bin
                  drivers/gpu/drm/i915/firmware/adlp_guc*.bin
                  drivers/gpu/drm/i915/firmware/adlp_huc*.bin"

    # WiFi firmware (RTL8852BE)
    ["wifi/rtw89"]="drivers/net/wireless/realtek/rtw89/rtw8852b_fw.bin
                    drivers/net/wireless/realtek/rtw89/rtw8852b_rfk.bin"

    # Network firmware (RTL8168)
    ["net/r8169"]="drivers/net/ethernet/realtek/r8169_firmware.bin"

    # Bluetooth firmware (Realtek)
    ["bluetooth/rtk"]="drivers/bluetooth/rtl8852b_config.bin
                      drivers/bluetooth/rtl8852b_fw.bin"
)

# Extract firmware files
for dir in "${!FIRMWARE[@]}"; do
    echo "Extracting firmware for $dir..."
    mkdir -p "$FIRMWARE_DIR/$dir"

    for pattern in ${FIRMWARE[$dir]}; do
        if [ -f "$LINUX_SRC/$pattern" ]; then
            cp "$LINUX_SRC/$pattern" "$FIRMWARE_DIR/$dir/"
            echo "  Extracted $(basename $pattern)"
        else
            # Try to find in compiled firmware directory
            fw_file=$(basename $pattern)
            if [ -f "$LINUX_SRC/../linux-firmware/$fw_file" ]; then
                cp "$LINUX_SRC/../linux-firmware/$fw_file" "$FIRMWARE_DIR/$dir/"
                echo "  Extracted $fw_file from linux-firmware"
            else
                echo "  Warning: Could not find $fw_file"
            fi
        fi
    done

    # Create firmware manifest
    {
        echo "# Firmware manifest for $dir"
        echo "# Extracted from Linux 6.14.4"
        echo "Files:"
        ls -1 "$FIRMWARE_DIR/$dir" 2>/dev/null
    } > "$FIRMWARE_DIR/$dir/manifest.txt"
done

echo "Firmware extraction complete. Check individual directories for extracted files."
