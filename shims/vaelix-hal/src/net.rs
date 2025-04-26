//! Network Hardware Abstraction Layer
//!
//! Provides abstractions for:
//! - Realtek RTL8852BE PCIe 802.11ax Wireless (2.5 GT/s)
//! - Realtek RTL8111/8168 PCIe Gigabit Ethernet (2.5 GT/s)

use crate::HalError;
use core::sync::atomic::{AtomicBool, Ordering};

// Interface state tracking
static WIFI_INITIALIZED: AtomicBool = AtomicBool::new(false);
static ETH_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Supported network interfaces
#[derive(Debug, Clone, Copy)]
pub enum Interface {
    WiFi,
    Ethernet,
}

/// Network interface statistics
#[derive(Debug, Clone)]
pub struct InterfaceStats {
    bytes_received: u64,
    bytes_sent: u64,
    packets_received: u64,
    packets_sent: u64,
    errors_in: u32,
    errors_out: u32,
}

/// WiFi security types
#[derive(Debug, Clone, Copy)]
pub enum SecurityType {
    None,
    WEP,
    WPA2Personal,
    WPA3Personal,
}

/// WiFi connection configuration
#[derive(Debug, Clone)]
pub struct WifiConfig {
    ssid: [u8; 32],
    password: [u8; 64],
    security: SecurityType,
}

/// Initialize network subsystem
pub(crate) fn init() -> Result<(), HalError> {
    // Initialize WiFi if available
    #[cfg(feature = "rtl8852be")]
    init_wifi()?;

    // Initialize Ethernet if available
    #[cfg(feature = "rtl8168")]
    init_ethernet()?;

    Ok(())
}

/// Shut down network subsystem
pub(crate) fn shutdown() -> Result<(), HalError> {
    if WIFI_INITIALIZED.load(Ordering::SeqCst) {
        #[cfg(feature = "rtl8852be")]
        shutdown_wifi()?;
    }

    if ETH_INITIALIZED.load(Ordering::SeqCst) {
        #[cfg(feature = "rtl8168")]
        shutdown_ethernet()?;
    }

    Ok(())
}

#[cfg(feature = "rtl8852be")]
fn init_wifi() -> Result<(), HalError> {
    // TODO: Initialize RTL8852BE
    // 1. Load firmware from Linux driver
    // 2. Configure PCIe interface
    // 3. Initialize hardware
    WIFI_INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(feature = "rtl8168")]
fn init_ethernet() -> Result<(), HalError> {
    // TODO: Initialize RTL8168
    // 1. Load firmware from Linux driver
    // 2. Configure PCIe interface
    // 3. Initialize hardware
    ETH_INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg(feature = "rtl8852be")]
fn shutdown_wifi() -> Result<(), HalError> {
    // TODO: Implement WiFi shutdown
    WIFI_INITIALIZED.store(false, Ordering::SeqCst);
    Ok(())
}

#[cfg(feature = "rtl8168")]
fn shutdown_ethernet() -> Result<(), HalError> {
    // TODO: Implement Ethernet shutdown
    ETH_INITIALIZED.store(false, Ordering::SeqCst);
    Ok(())
}

/// Get interface status
pub fn get_status(interface: Interface) -> Result<bool, HalError> {
    match interface {
        Interface::WiFi => {
            #[cfg(feature = "rtl8852be")]
            return Ok(WIFI_INITIALIZED.load(Ordering::SeqCst));
            #[cfg(not(feature = "rtl8852be"))]
            return Err(HalError::UnsupportedHardware);
        }
        Interface::Ethernet => {
            #[cfg(feature = "rtl8168")]
            return Ok(ETH_INITIALIZED.load(Ordering::SeqCst));
            #[cfg(not(feature = "rtl8168"))]
            return Err(HalError::UnsupportedHardware);
        }
    }
}

/// Get interface statistics
pub fn get_stats(interface: Interface) -> Result<InterfaceStats, HalError> {
    match interface {
        Interface::WiFi => {
            if !WIFI_INITIALIZED.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }
            // TODO: Implement WiFi statistics collection
        }
        Interface::Ethernet => {
            if !ETH_INITIALIZED.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }
            // TODO: Implement Ethernet statistics collection
        }
    }

    Ok(InterfaceStats {
        bytes_received: 0,
        bytes_sent: 0,
        packets_received: 0,
        packets_sent: 0,
        errors_in: 0,
        errors_out: 0,
    })
}

/// Configure WiFi connection
#[cfg(feature = "rtl8852be")]
pub fn configure_wifi(config: WifiConfig) -> Result<(), HalError> {
    if !WIFI_INITIALIZED.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }
    // TODO: Implement WiFi configuration
    Ok(())
}

/// Enable or disable power management for an interface
pub fn set_power_saving(interface: Interface, enable: bool) -> Result<(), HalError> {
    match interface {
        Interface::WiFi => {
            if !WIFI_INITIALIZED.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }
            // TODO: Implement WiFi power management
        }
        Interface::Ethernet => {
            if !ETH_INITIALIZED.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }
            // TODO: Implement Ethernet power management
        }
    }
    Ok(())
}
