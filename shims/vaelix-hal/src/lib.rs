#![no_std]
#![feature(error_in_core)]

//! VaelixOS Hardware Abstraction Layer
//!
//! This crate provides safe Rust abstractions over hardware drivers,
//! implementing shims for Linux driver functionality where needed.

pub mod audio;
pub mod bluetooth;
pub mod cpu;
pub mod gpu;
pub mod net;
pub mod storage;

use core::error::Error;
use core::fmt;

/// Hardware abstraction layer errors
#[derive(Debug)]
pub enum HalError {
    NotInitialized,
    UnsupportedHardware,
    DeviceError,
    IoError,
    BufferError,
}

impl Error for HalError {}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HalError::NotInitialized => write!(f, "Hardware not initialized"),
            HalError::UnsupportedHardware => write!(f, "Hardware not supported"),
            HalError::DeviceError => write!(f, "Device error occurred"),
            HalError::IoError => write!(f, "I/O error occurred"),
            HalError::BufferError => write!(f, "Buffer error occurred"),
        }
    }
}

/// Initialize all hardware subsystems
pub fn init() -> Result<(), HalError> {
    // Initialize subsystems in dependency order
    cpu::init()?;
    storage::init()?;
    gpu::init()?;
    audio::init()?;
    net::init()?;
    bluetooth::init()?;

    Ok(())
}

/// Shut down all hardware subsystems
pub fn shutdown() -> Result<(), HalError> {
    // Shutdown in reverse order of initialization
    bluetooth::shutdown()?;
    net::shutdown()?;
    audio::shutdown()?;
    gpu::shutdown()?;
    storage::shutdown()?;
    cpu::shutdown()?;

    Ok(())
}
