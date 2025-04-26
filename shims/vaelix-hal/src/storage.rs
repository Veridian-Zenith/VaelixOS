//! Storage Hardware Abstraction Layer
//!
//! Provides abstractions for storage devices, specifically targeting
//! NVMe SSDs with PCIe Gen3 x4 interface (63.2 Gb/s)

use crate::HalError;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

/// Track storage device state
static DEVICE_INITIALIZED: AtomicBool = AtomicBool::new(false);
static AVAILABLE_SPACE: AtomicU64 = AtomicU64::new(0);

/// Storage device capabilities
#[derive(Debug, Clone)]
pub struct StorageCapabilities {
    total_size: u64,         // Total size in bytes
    max_transfer_speed: u64, // Maximum transfer speed in bytes/sec
    supports_trim: bool,     // TRIM command support
    supports_smart: bool,    // SMART monitoring support
}

/// Storage device power states
#[derive(Debug, Clone, Copy)]
pub enum PowerState {
    Active,     // Full power operation
    LowPower,   // Reduced power operation
    Standby,    // Minimal power state
}

/// Storage operation modes
#[derive(Debug, Clone, Copy)]
pub enum OperationMode {
    Normal,     // Standard operation
    SafeMode,   // Conservative settings for reliability
    Performance // Optimized for speed
}

/// Initialize storage subsystem
pub(crate) fn init() -> Result<(), HalError> {
    #[cfg(feature = "nvme")]
    {
        // Initialize NVMe controller
        init_nvme_controller()?;

        // Set up DMA regions
        init_dma()?;

        // Initialize power management
        init_power_management()?;

        DEVICE_INITIALIZED.store(true, Ordering::SeqCst);
        // Set initial available space (238.47 GiB from sys.txt)
        AVAILABLE_SPACE.store(256_060_514_304, Ordering::SeqCst);

        Ok(())
    }

    #[cfg(not(feature = "nvme"))]
    Err(HalError::UnsupportedHardware)
}

/// Shut down storage subsystem
pub(crate) fn shutdown() -> Result<(), HalError> {
    if !DEVICE_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }

    #[cfg(feature = "nvme")]
    {
        // Flush all caches
        flush_caches()?;

        // Set to safe power state
        set_power_state(PowerState::Standby)?;

        DEVICE_INITIALIZED.store(false, Ordering::SeqCst);
        Ok(())
    }

    #[cfg(not(feature = "nvme"))]
    Err(HalError::UnsupportedHardware)
}

#[cfg(feature = "nvme")]
fn init_nvme_controller() -> Result<(), HalError> {
    // TODO: Initialize NVMe controller using extracted Linux driver code
    // This will handle PCIe setup and controller initialization
    Ok(())
}

#[cfg(feature = "nvme")]
fn init_dma() -> Result<(), HalError> {
    // TODO: Set up DMA regions for NVMe transfers
    Ok(())
}

#[cfg(feature = "nvme")]
fn init_power_management() -> Result<(), HalError> {
    // TODO: Initialize power management features
    Ok(())
}

#[cfg(feature = "nvme")]
fn flush_caches() -> Result<(), HalError> {
    // TODO: Implement cache flushing
    Ok(())
}

/// Set storage device power state
#[cfg(feature = "nvme")]
pub fn set_power_state(state: PowerState) -> Result<(), HalError> {
    if !DEVICE_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }
    // TODO: Implement power state management using NVMe features
    Ok(())
}

/// Get storage device capabilities
#[cfg(feature = "nvme")]
pub fn get_capabilities() -> Result<StorageCapabilities, HalError> {
    if !DEVICE_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }

    Ok(StorageCapabilities {
        total_size: 256_060_514_304,  // 238.47 GiB
        max_transfer_speed: 7_900_000_000, // 63.2 Gb/s
        supports_trim: true,
        supports_smart: true,
    })
}

/// Set operation mode
#[cfg(feature = "nvme")]
pub fn set_operation_mode(mode: OperationMode) -> Result<(), HalError> {
    if !DEVICE_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }
    // TODO: Implement operation mode switching
    Ok(())
}

/// Get available space
#[cfg(feature = "nvme")]
pub fn get_available_space() -> Result<u64, HalError> {
    if !DEVICE_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }
    Ok(AVAILABLE_SPACE.load(Ordering::SeqCst))
}
