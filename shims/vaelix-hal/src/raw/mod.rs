//! Raw hardware interaction layer
//!
//! Provides safe abstractions for hardware register access and memory-mapped I/O.
//! This module implements the core functionality needed by the HAL shims.

#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

/// A memory-mapped register
///
/// This struct represents a memory-mapped register.
#[derive(Debug)]
#[repr(transparent)]
pub struct Register<T> {
    /// Value of the register
    value: T,
}

impl<T> Register<T> {
    /// Create a new register at the given address
    ///
    /// This function creates a new register at the given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the register.
    ///
    /// # Returns
    ///
    /// * `&'static mut Self` - A mutable reference to the new register.
    pub const unsafe fn new(addr: usize) -> &'static mut Self {
        &mut *(addr as *mut Self)
    }

    /// Read the register value
    ///
    /// This function reads the value of the register.
    ///
    /// # Returns
    ///
    /// * `T` - The value of the register.
    pub fn read(&self) -> T
    where
        T: Copy,
    {
        unsafe { read_volatile(&self.value) }
    }

    /// Write a value to the register
    ///
    /// This function writes a value to the register.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to write.
    pub fn write(&mut self, value: T) {
        unsafe { write_volatile(&mut self.value, value) }
    }

    /// Modify the register value using a closure
    ///
    /// This function modifies the register value using a closure.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes the current value and returns the new value.
    pub fn modify<F>(&mut self, f: F)
    where
        T: Copy,
        F: FnOnce(T) -> T,
    {
        let value = self.read();
        self.write(f(value));
    }
}

/// Memory-mapped I/O region
///
/// This struct represents a memory-mapped I/O region.
#[derive(Debug)]
pub struct IoRegion {
    /// Base address of the I/O region
    base: usize,
    /// Size of the I/O region
    size: usize,
}

impl IoRegion {
    /// Create a new I/O region
    ///
    /// This function creates a new I/O region.
    ///
    /// # Arguments
    ///
    /// * `base` - The base address of the I/O region.
    /// * `size` - The size of the I/O region.
    ///
    /// # Returns
    ///
    /// * `Self` - The new I/O region.
    pub const unsafe fn new(base: usize, size: usize) -> Self {
        Self { base, size }
    }

    /// Read a value from an offset in the I/O region
    ///
    /// This function reads a value from an offset in the I/O region.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the I/O region.
    ///
    /// # Returns
    ///
    /// * `T` - The value read from the I/O region.
    pub fn read<T>(&self, offset: usize) -> T
    where
        T: Copy,
    {
        assert!(offset + core::mem::size_of::<T>() <= self.size);
        unsafe { read_volatile((self.base + offset) as *const T) }
    }

    /// Write a value to an offset in the I/O region
    ///
    /// This function writes a value to an offset in the I/O region.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the I/O region.
    /// * `value` - The value to write.
    pub fn write<T>(&mut self, offset: usize, value: T) {
        assert!(offset + core::mem::size_of::<T>() <= self.size);
        unsafe {
            write_volatile((self.base + offset) as *mut T, value);
        }
    }

    /// Get a register at the given offset
    ///
    /// This function gets a register at the given offset in the I/O region.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset in the I/O region.
    ///
    /// # Returns
    ///
    /// * `&'static mut Register<T>` - A mutable reference to the register.
    pub fn register<T>(&self, offset: usize) -> &'static mut Register<T> {
        assert!(offset + core::mem::size_of::<T>() <= self.size);
        unsafe { Register::new(self.base + offset) }
    }
}

/// PCI configuration space access
///
/// This module provides access to the PCI configuration space.
pub mod pci {
    use super::*;

    /// PCI configuration address port
    const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
    /// PCI configuration data port
    const PCI_CONFIG_DATA: u16 = 0xCFC;

    /// Read from PCI configuration space
    ///
    /// This function reads from the PCI configuration space.
    ///
    /// # Arguments
    ///
    /// * `bus` - The bus number.
    /// * `slot` - The slot number.
    /// * `func` - The function number.
    /// * `offset` - The offset in the configuration space.
    ///
    /// # Returns
    ///
    /// * `u32` - The value read from the configuration space.
    pub fn read_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        let address = ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC)
            | 0x80000000;

        unsafe {
            port_write(PCI_CONFIG_ADDRESS, address);
            port_read(PCI_CONFIG_DATA)
        }
    }

    /// Write to PCI configuration space
    ///
    /// This function writes to the PCI configuration space.
    ///
    /// # Arguments
    ///
    /// * `bus` - The bus number.
    /// * `slot` - The slot number.
    /// * `func` - The function number.
    /// * `offset` - The offset in the configuration space.
    /// * `value` - The value to write.
    pub fn write_config(bus: u8, slot: u8, func: u8, offset: u8, value: u32) {
        let address = ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((offset as u32) & 0xFC)
            | 0x80000000;

        unsafe {
            port_write(PCI_CONFIG_ADDRESS, address);
            port_write(PCI_CONFIG_DATA, value);
        }
    }
}

/// I/O port operations
///
/// This module provides operations for I/O ports.

/// Read from an I/O port
///
/// This function reads from an I/O port.
///
/// # Arguments
///
/// * `port` - The port number.
///
/// # Returns
///
/// * `T` - The value read from the I/O port.
pub unsafe fn port_read<T>(port: u16) -> T
where
    T: Copy,
{
    let mut value: T = core::mem::zeroed();
    match core::mem::size_of::<T>() {
        1 => asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack)),
        2 => asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack)),
        4 => asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack)),
        _ => panic!("Invalid port read size"),
    }
    fence(Ordering::Acquire);
    value
}

/// Write to an I/O port
///
/// This function writes to an I/O port.
///
/// # Arguments
///
/// * `port` - The port number.
/// * `value` - The value to write.
pub unsafe fn port_write<T>(port: u16, value: T)
where
    T: Copy,
{
    fence(Ordering::Release);
    match core::mem::size_of::<T>() {
        1 => asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack)),
        2 => asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack)),
        4 => asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack)),
        _ => panic!("Invalid port write size"),
    }
}
