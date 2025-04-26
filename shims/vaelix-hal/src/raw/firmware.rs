//! Firmware Loading Management
//!
//! Handles the loading and management of firmware files extracted from Linux drivers:
//! - RTL8852BE WiFi firmware (rtw8852b_fw.bin)
//! - Intel i915 GPU firmware (various DMC and GUC firmwares)

use crate::HalError;
use core::sync::atomic::{AtomicBool, Ordering};
use alloc::vec::Vec;

/// Maximum firmware size supported (16MB)
///
/// This constant defines the maximum size of firmware that is supported.
#[derive(Debug)]
const MAX_FIRMWARE_SIZE: usize = 16 * 1024 * 1024;

/// Firmware loading state
///
/// This enum defines the possible states of firmware loading.
#[derive(Debug, Clone, Copy)]
pub enum FirmwareState {
    /// Not loaded state
    NotLoaded,
    /// Loading state
    Loading,
    /// Ready state
    Ready,
    /// Error state
    Error,
}

/// Firmware descriptor
///
/// This struct represents the descriptor of a firmware file.
#[derive(Debug)]
pub struct FirmwareDesc {
    /// Name of the firmware
    pub name: &'static str,
    /// Device ID associated with the firmware
    pub device_id: u16,
    /// Version of the firmware
    pub version: u32,
    /// Flags associated with the firmware
    pub flags: u32,
}

/// Firmware instance
///
/// This struct represents an instance of firmware.
#[derive(Debug)]
pub struct Firmware {
    /// Descriptor of the firmware
    desc: FirmwareDesc,
    /// Data of the firmware
    data: Vec<u8>,
    /// State of the firmware
    state: AtomicBool,
}

impl Firmware {
    /// Create new firmware instance
    ///
    /// This function creates a new instance of firmware.
    ///
    /// # Arguments
    ///
    /// * `desc` - The descriptor of the firmware.
    ///
    /// # Returns
    ///
    /// * `Self` - The new firmware instance.
    pub fn new(desc: FirmwareDesc) -> Self {
        Self {
            desc,
            data: Vec::new(),
            state: AtomicBool::new(false),
        }
    }

    /// Load firmware data
    ///
    /// This function loads the firmware data. It checks if the data size exceeds the maximum supported size and returns an error if it does.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to load.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn load(&mut self, data: &[u8]) -> Result<(), HalError> {
        if data.len() > MAX_FIRMWARE_SIZE {
            return Err(HalError::BufferError);
        }

        self.data.clear();
        self.data.extend_from_slice(data);
        self.state.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Get firmware data
    ///
    /// This function returns the firmware data if it is loaded.
    ///
    /// # Returns
    ///
    /// * `Option<&[u8]>` - An option containing the firmware data or None if not loaded.
    pub fn data(&self) -> Option<&[u8]> {
        if !self.state.load(Ordering::SeqCst) {
            return None;
        }
        Some(&self.data)
    }

    /// Check if firmware is loaded
    ///
    /// This function checks if the firmware is loaded.
    ///
    /// # Returns
    ///
    /// * `bool` - A boolean indicating whether the firmware is loaded.
    pub fn is_loaded(&self) -> bool {
        self.state.load(Ordering::SeqCst)
    }
}

/// Known firmware files
///
/// These constants define the known firmware files.
#[derive(Debug)]
pub const RTW8852B_FW: FirmwareDesc = FirmwareDesc {
    name: "rtw8852b_fw.bin",
    device_id: 0xb852,
    version: 1,
    flags: 0,
};

#[derive(Debug)]
pub const I915_DMC_FW: FirmwareDesc = FirmwareDesc {
    name: "adlp_dmc.bin",
    device_id: 0x46b3,
    version: 1,
    flags: 0,
};

/// Firmware cache to avoid reloading
///
/// This static variable represents the firmware cache to avoid reloading.
static mut FIRMWARE_CACHE: Option<alloc::collections::BTreeMap<u16, Firmware>> = None;

/// Initialize firmware subsystem
///
/// This function initializes the firmware subsystem. It creates a new firmware cache.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn init() -> Result<(), HalError> {
    unsafe {
        FIRMWARE_CACHE = Some(alloc::collections::BTreeMap::new());
    }
    Ok(())
}

/// Load firmware for a device
///
/// This function loads the firmware for a device. It checks if the firmware is already loaded and returns it if it is. Otherwise, it creates a new firmware instance, loads the data, and stores it in the cache.
///
/// # Arguments
///
/// * `device_id` - The device ID.
/// * `fw_desc` - The firmware descriptor.
///
/// # Returns
///
/// * `Result<&'static Firmware, HalError>` - A result containing the firmware or an error.
pub fn load_firmware(device_id: u16, fw_desc: &FirmwareDesc) -> Result<&'static Firmware, HalError> {
    unsafe {
        let cache = FIRMWARE_CACHE.as_mut().ok_or(HalError::NotInitialized)?;

        // Check if already loaded
        if let Some(fw) = cache.get(&device_id) {
            return Ok(fw);
        }

        // Create new firmware instance
        let mut fw = Firmware::new(fw_desc.clone());

        // Load firmware data from Linux driver directory
        // TODO: Extract and load actual firmware data
        fw.load(&[])?;

        // Store in cache
        cache.insert(device_id, fw);

        Ok(cache.get(&device_id).unwrap())
    }
}

/// Unload firmware for a device
///
/// This function unloads the firmware for a device. It removes the firmware from the cache.
///
/// # Arguments
///
/// * `device_id` - The device ID.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn unload_firmware(device_id: u16) -> Result<(), HalError> {
    unsafe {
        let cache = FIRMWARE_CACHE.as_mut().ok_or(HalError::NotInitialized)?;
        cache.remove(&device_id);
    }
    Ok(())
}

/// Get loaded firmware
///
/// This function returns the loaded firmware for a device.
///
/// # Arguments
///
/// * `device_id` - The device ID.
///
/// # Returns
///
/// * `Option<&'static Firmware>` - An option containing the firmware or None if not loaded.
pub fn get_firmware(device_id: u16) -> Option<&'static Firmware> {
    unsafe {
        FIRMWARE_CACHE.as_ref()?.get(&device_id)
    }
}
