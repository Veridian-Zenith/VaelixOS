//! Runtime Firmware Loading
//!
//! Provides runtime firmware loading capabilities from the system
//! instead of embedding firmware blobs directly.

use crate::HalError;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Default firmware search paths
///
/// This constant defines the default paths where firmware files are searched for.
#[derive(Debug)]
const FIRMWARE_PATHS: &[&str] = &[
    "/lib/firmware",
    "/usr/lib/firmware",
    "/lib/firmware/updates",
];

/// Runtime firmware instance
///
/// This struct represents an instance of runtime firmware.
#[derive(Debug)]
pub struct RuntimeFirmware {
    /// Name of the firmware
    name: &'static str,
    /// Data of the firmware
    data: Vec<u8>,
    /// Loaded flag
    loaded: AtomicBool,
}

impl RuntimeFirmware {
    /// Create new runtime firmware instance
    ///
    /// This function creates a new instance of runtime firmware.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the firmware.
    ///
    /// # Returns
    ///
    /// * `Self` - The new runtime firmware instance.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: Vec::new(),
            loaded: AtomicBool::new(false),
        }
    }

    /// Load firmware from system
    ///
    /// This function loads the firmware from the system. It checks if the firmware is already loaded and returns an error if it is.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn load(&mut self) -> Result<(), HalError> {
        if self.loaded.load(Ordering::SeqCst) {
            return Ok(());
        }

        // TODO: Implement actual filesystem access
        // For now, we'll create placeholder data
        self.data.clear();
        match self.name {
            "rtw8852b_fw.bin" => {
                // WiFi firmware placeholder
                self.data.extend_from_slice(&[0xFF; 1024]);
            }
            "adlp_dmc.bin" => {
                // GPU firmware placeholder
                self.data.extend_from_slice(&[0xEE; 1024]);
            }
            _ => return Err(HalError::DeviceError),
        }

        self.loaded.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Get firmware data if loaded
    ///
    /// This function returns the firmware data if it is loaded.
    ///
    /// # Returns
    ///
    /// * `Option<&[u8]>` - An option containing the firmware data or None if not loaded.
    pub fn data(&self) -> Option<&[u8]> {
        if self.loaded.load(Ordering::SeqCst) {
            Some(&self.data)
        } else {
            None
        }
    }

    /// Check if firmware is loaded
    ///
    /// This function checks if the firmware is loaded.
    ///
    /// # Returns
    ///
    /// * `bool` - A boolean indicating whether the firmware is loaded.
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }

    /// Get firmware name
    ///
    /// This function returns the name of the firmware.
    ///
    /// # Returns
    ///
    /// * `&str` - The name of the firmware.
    pub fn name(&self) -> &str {
        self.name
    }
}

/// Known firmware files and their fallback paths
///
/// This constant defines the known firmware files and their fallback paths.
#[derive(Debug)]
pub const FIRMWARE_INFO: &[(&str, &[&str])] = &[
    // WiFi firmware
    ("rtw8852b_fw.bin", &[
        "rtw89/rtw8852b_fw.bin",
        "rtlwifi/rtw8852b_fw.bin",
    ]),

    // GPU firmware
    ("adlp_dmc.bin", &[
        "i915/adlp_dmc.bin",
        "intel/adlp_dmc.bin",
    ]),
];

/// Global firmware registry
///
/// This static variable represents the global firmware registry.
static mut FIRMWARE_REGISTRY: Option<alloc::collections::BTreeMap<&'static str, RuntimeFirmware>> = None;

/// Initialize runtime firmware system
///
/// This function initializes the runtime firmware system. It creates a new firmware registry.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn init() -> Result<(), HalError> {
    unsafe {
        FIRMWARE_REGISTRY = Some(alloc::collections::BTreeMap::new());
    }
    Ok(())
}

/// Register firmware for loading
///
/// This function registers firmware for loading. It adds the firmware to the registry if it is not already present.
///
/// # Arguments
///
/// * `name` - The name of the firmware.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn register_firmware(name: &'static str) -> Result<(), HalError> {
    unsafe {
        let registry = FIRMWARE_REGISTRY.as_mut().ok_or(HalError::NotInitialized)?;

        if !registry.contains_key(name) {
            registry.insert(name, RuntimeFirmware::new(name));
        }

        Ok(())
    }
}

/// Request firmware loading
///
/// This function requests the loading of firmware. It loads the firmware from the registry and returns it.
///
/// # Arguments
///
/// * `name` - The name of the firmware.
///
/// # Returns
///
/// * `Result<&'static RuntimeFirmware, HalError>` - A result containing the firmware or an error.
pub fn request_firmware(name: &str) -> Result<&'static RuntimeFirmware, HalError> {
    unsafe {
        let registry = FIRMWARE_REGISTRY.as_mut().ok_or(HalError::NotInitialized)?;

        let fw = registry.get_mut(name).ok_or(HalError::DeviceError)?;
        fw.load()?;

        Ok(registry.get(name).unwrap())
    }
}

/// Check if firmware is available
///
/// This function checks if firmware is available. It returns true if the firmware is available, false otherwise.
///
/// # Arguments
///
/// * `name` - The name of the firmware.
///
/// # Returns
///
/// * `bool` - A boolean indicating whether the firmware is available.
pub fn is_firmware_available(name: &str) -> bool {
    // TODO: Implement actual filesystem check
    FIRMWARE_INFO.iter().any(|(fw_name, _)| *fw_name == name)
}
