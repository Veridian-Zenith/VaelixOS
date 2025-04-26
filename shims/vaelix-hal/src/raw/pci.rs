//! PCI device management
//!
//! Handles PCI device discovery and configuration for:
//! - Intel UHD Graphics (8086:46b3)
//! - Realtek RTL8852BE WiFi (10ec:b852)
//! - Realtek RTL8111/8168 Ethernet (10ec:8168)
//! - NVMe Controller

use super::{IoRegion, Register};
use crate::HalError;

/// PCI address structure
///
/// This struct represents a PCI address.
#[derive(Debug, Clone, Copy)]
pub struct PciAddress {
    /// Bus number
    pub bus: u8,
    /// Slot number
    pub slot: u8,
    /// Function number
    pub func: u8,
}

/// PCI device structure
///
/// This struct represents a PCI device.
#[derive(Debug, Clone)]
pub struct PciDevice {
    /// Address of the PCI device
    pub address: PciAddress,
    /// Vendor ID of the PCI device
    pub vendor_id: u16,
    /// Device ID of the PCI device
    pub device_id: u16,
    /// Class of the PCI device
    pub class: u8,
    /// Subclass of the PCI device
    pub subclass: u8,
    /// Programming interface of the PCI device
    pub prog_if: u8,
    /// Header type of the PCI device
    pub header_type: u8,
}

impl PciDevice {
    /// Read from device's PCI configuration space
    ///
    /// This function reads from the device's PCI configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the configuration space.
    ///
    /// # Returns
    ///
    /// * `u32` - The value read from the configuration space.
    pub fn read_config(&self, offset: u8) -> u32 {
        super::pci::read_config(
            self.address.bus,
            self.address.slot,
            self.address.func,
            offset,
        )
    }

    /// Write to device's PCI configuration space
    ///
    /// This function writes to the device's PCI configuration space.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the configuration space.
    /// * `value` - The value to write.
    pub fn write_config(&self, offset: u8, value: u32) {
        super::pci::write_config(
            self.address.bus,
            self.address.slot,
            self.address.func,
            offset,
            value,
        )
    }

    /// Get device's BAR (Base Address Register)
    ///
    /// This function gets the device's Base Address Register (BAR).
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the BAR.
    ///
    /// # Returns
    ///
    /// * `Option<IoRegion>` - An option containing the IoRegion or None if the BAR is not valid.
    pub fn get_bar(&self, index: u8) -> Option<IoRegion> {
        if index >= 6 {
            return None;
        }

        let bar = self.read_config(0x10 + index * 4);
        if bar == 0 {
            return None;
        }

        // Check if this is memory BAR
        if bar & 1 == 0 {
            let base = (bar & !0xF) as usize;

            // Write all 1s to determine size
            self.write_config(0x10 + index * 4, 0xFFFFFFFF);
            let size_mask = self.read_config(0x10 + index * 4);
            // Restore original value
            self.write_config(0x10 + index * 4, bar);

            let size = !((size_mask & !0xF) as usize) + 1;

            Some(unsafe { IoRegion::new(base, size) })
        } else {
            // I/O BAR not supported yet
            None
        }
    }

    /// Enable bus mastering
    ///
    /// This function enables bus mastering for the device.
    pub fn enable_bus_master(&self) {
        let cmd = self.read_config(0x04);
        self.write_config(0x04, cmd | 0x4);
    }

    /// Enable memory space access
    ///
    /// This function enables memory space access for the device.
    pub fn enable_memory_space(&self) {
        let cmd = self.read_config(0x04);
        self.write_config(0x04, cmd | 0x2);
    }
}

/// Invalid vendor ID
///
/// This constant represents an invalid vendor ID.
#[derive(Debug)]
const INVALID_VENDOR: u16 = 0xFFFF;

/// Scan PCI bus for devices
///
/// This function scans the PCI bus for devices.
///
/// # Returns
///
/// * `impl Iterator<Item = PciDevice>` - An iterator over the PCI devices found.
pub fn scan_devices() -> impl Iterator<Item = PciDevice> {
    PciDeviceIter::new()
}

/// PCI device iterator
///
/// This struct represents an iterator for PCI devices.
#[derive(Debug)]
struct PciDeviceIter {
    /// Next address to scan
    next_addr: PciAddress,
}

impl PciDeviceIter {
    /// Create a new PCI device iterator
    ///
    /// This function creates a new PCI device iterator.
    ///
    /// # Returns
    ///
    /// * `Self` - The new PCI device iterator.
    fn new() -> Self {
        Self {
            next_addr: PciAddress {
                bus: 0,
                slot: 0,
                func: 0,
            },
        }
    }
}

impl Iterator for PciDeviceIter {
    type Item = PciDevice;

    /// Get the next PCI device
    ///
    /// This function gets the next PCI device in the iterator.
    ///
    /// # Returns
    ///
    /// * `Option<Self::Item>` - An option containing the next PCI device or None if there are no more devices.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let addr = self.next_addr;

            // Move to next address
            self.next_addr.func += 1;
            if self.next_addr.func >= 8 {
                self.next_addr.func = 0;
                self.next_addr.slot += 1;
                if self.next_addr.slot >= 32 {
                    self.next_addr.slot = 0;
                    self.next_addr.bus += 1;
                    if self.next_addr.bus >= 256 {
                        return None;
                    }
                }
            }

            let config = super::pci::read_config(addr.bus, addr.slot, addr.func, 0);
            let vendor_id = (config & 0xFFFF) as u16;

            if vendor_id != INVALID_VENDOR {
                let device_id = ((config >> 16) & 0xFFFF) as u16;
                let class_info = super::pci::read_config(addr.bus, addr.slot, addr.func, 8);

                return Some(PciDevice {
                    address: addr,
                    vendor_id,
                    device_id,
                    class: ((class_info >> 24) & 0xFF) as u8,
                    subclass: ((class_info >> 16) & 0xFF) as u8,
                    prog_if: ((class_info >> 8) & 0xFF) as u8,
                    header_type: ((super::pci::read_config(addr.bus, addr.slot, addr.func, 0x0C)
                        >> 16)
                        & 0xFF) as u8,
                });
            }
        }
    }
}

/// Known PCI device IDs
///
/// These constants represent known PCI device IDs.
#[derive(Debug)]
pub const INTEL_GPU_VENDOR: u16 = 0x8086;
#[derive(Debug)]
pub const INTEL_GPU_DEVICE: u16 = 0x46b3; // Alder Lake-UP3 GT1
#[derive(Debug)]
pub const REALTEK_VENDOR: u16 = 0x10ec;
#[derive(Debug)]
pub const RTL8852BE_DEVICE: u16 = 0xb852; // RTL8852BE WiFi
#[derive(Debug)]
pub const RTL8168_DEVICE: u16 = 0x8168; // RTL8168/8111 Ethernet

/// Find a specific PCI device by vendor and device ID
///
/// This function finds a specific PCI device by vendor and device ID.
///
/// # Arguments
///
/// * `vendor_id` - The vendor ID of the device.
/// * `device_id` - The device ID of the device.
///
/// # Returns
///
/// * `Option<PciDevice>` - An option containing the PCI device or None if not found.
pub fn find_device(vendor_id: u16, device_id: u16) -> Option<PciDevice> {
    scan_devices().find(|dev| dev.vendor_id == vendor_id && dev.device_id == device_id)
}

/// Initialize a PCI device with memory access and bus mastering
///
/// This function initializes a PCI device with memory access and bus mastering.
///
/// # Arguments
///
/// * `device` - A reference to the PCI device.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn init_device(device: &PciDevice) -> Result<(), HalError> {
    device.enable_memory_space();
    device.enable_bus_master();
    Ok(())
}
