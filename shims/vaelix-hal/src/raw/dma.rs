//! DMA Management Layer
//!
//! Provides DMA (Direct Memory Access) management for device drivers:
//! - Memory allocation and mapping
//! - Scatter-gather list management
//! - Coherent DMA operations
//! Based on Linux DMA-API

use crate::HalError;
use core::sync::atomic::{AtomicPtr, AtomicBool, Ordering};
use alloc::vec::Vec;

/// DMA direction for data transfer
///
/// This enum defines the possible directions for DMA data transfer.
#[derive(Debug, Clone, Copy)]
pub enum DmaDirection {
    /// Transfer to device
    ToDevice,
    /// Transfer from device
    FromDevice,
    /// Bidirectional transfer
    Bidirectional,
}

/// DMA address mapping
///
/// This struct represents the mapping of a DMA address.
#[derive(Debug)]
pub struct DmaMapping {
    /// Virtual address
    virt_addr: *mut u8,
    /// Physical address
    phys_addr: usize,
    /// Size of the mapping
    size: usize,
    /// Direction of the DMA transfer
    direction: DmaDirection,
    /// Coherent flag
    coherent: bool,
}

/// Scatter-gather entry
///
/// This struct represents an entry in a scatter-gather list.
#[derive(Debug, Clone)]
pub struct ScatterGatherEntry {
    /// Address of the entry
    addr: usize,
    /// Length of the entry
    length: usize,
    /// Last entry flag
    last: bool,
}

/// DMA buffer flags
///
/// This bitflags struct defines the possible flags for a DMA buffer.
#[derive(Debug)]
bitflags::bitflags! {
    pub struct DmaFlags: u32 {
        /// Coherent flag
        const COHERENT = 1 << 0;
        /// Streaming flag
        const STREAMING = 1 << 1;
        /// Bounce flag
        const BOUNCE = 1 << 2;
        /// Zero copy flag
        const ZERO_COPY = 1 << 3;
    }
}

/// DMA allocation context
///
/// This struct represents the context for DMA allocation.
#[derive(Debug)]
pub struct DmaContext {
    /// DMA pool
    pool: AtomicPtr<u8>,
    /// Size of the DMA pool
    size: usize,
    /// Initialized flag
    initialized: AtomicBool,
    /// List of DMA mappings
    mappings: Vec<DmaMapping>,
}

// Singleton DMA context
static mut DMA_CTX: Option<DmaContext> = None;

/// Initialize DMA subsystem
///
/// This function initializes the DMA subsystem. It allocates a DMA pool and sets up the DMA context.
pub fn init() -> Result<(), HalError> {
    unsafe {
        if DMA_CTX.is_some() {
            return Ok(());
        }

        // Allocate DMA pool (16MB)
        let pool = alloc::alloc::alloc_zeroed(
            alloc::alloc::Layout::from_size_align(16 * 1024 * 1024, 4096)
                .map_err(|_| HalError::DeviceError)?
        );

        DMA_CTX = Some(DmaContext {
            pool: AtomicPtr::new(pool),
            size: 16 * 1024 * 1024,
            initialized: AtomicBool::new(true),
            mappings: Vec::new(),
        });

        Ok(())
    }
}

/// Allocate DMA buffer
///
/// This function allocates a DMA buffer. It allocates memory from the DMA pool and returns a pointer to the allocated buffer.
///
/// # Arguments
///
/// * `size` - The size of the buffer to allocate.
/// * `flags` - The flags for the buffer.
///
/// # Returns
///
/// * `Result<*mut u8, HalError>` - A result containing the pointer to the allocated buffer or an error.
pub fn alloc_coherent(size: usize, flags: DmaFlags) -> Result<*mut u8, HalError> {
    unsafe {
        let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctx.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // Align size to page boundary
        let aligned_size = (size + 4095) & !4095;

        // Allocate from pool
        let ptr = alloc::alloc::alloc_zeroed(
            alloc::alloc::Layout::from_size_align(aligned_size, 4096)
                .map_err(|_| HalError::DeviceError)?
        );

        Ok(ptr)
    }
}

/// Free DMA buffer
///
/// This function frees a DMA buffer. It deallocates the memory associated with the buffer.
///
/// # Arguments
///
/// * `ptr` - The pointer to the buffer to free.
/// * `size` - The size of the buffer.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub unsafe fn free_coherent(ptr: *mut u8, size: usize) -> Result<(), HalError> {
    let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
    if !ctx.initialized.load(Ordering::SeqCst) {
        return Err(HalError::NotInitialized);
    }

    // Align size to page boundary
    let aligned_size = (size + 4095) & !4095;

    alloc::alloc::dealloc(
        ptr,
        alloc::alloc::Layout::from_size_align(aligned_size, 4096)
            .map_err(|_| HalError::DeviceError)?
    );

    Ok(())
}

/// Map memory for DMA
///
/// This function maps memory for DMA. It creates a mapping between the virtual and physical addresses.
///
/// # Arguments
///
/// * `virt_addr` - The virtual address to map.
/// * `size` - The size of the memory to map.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<usize, HalError>` - A result containing the physical address or an error.
pub fn map_single(
    virt_addr: *mut u8,
    size: usize,
    direction: DmaDirection,
) -> Result<usize, HalError> {
    unsafe {
        let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctx.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // For now, we'll use identity mapping
        let phys_addr = virt_addr as usize;

        // Store mapping
        ctx.mappings.push(DmaMapping {
            virt_addr,
            phys_addr,
            size,
            direction,
            coherent: false,
        });

        Ok(phys_addr)
    }
}

/// Unmap DMA memory
///
/// This function unmaps DMA memory. It removes the mapping between the virtual and physical addresses.
///
/// # Arguments
///
/// * `phys_addr` - The physical address to unmap.
/// * `size` - The size of the memory to unmap.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn unmap_single(phys_addr: usize, size: usize, direction: DmaDirection) -> Result<(), HalError> {
    unsafe {
        let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctx.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // Find and remove mapping
        if let Some(pos) = ctx.mappings.iter().position(|m| {
            m.phys_addr == phys_addr && m.size == size && m.direction == direction
        }) {
            ctx.mappings.remove(pos);
        }

        Ok(())
    }
}

/// Create scatter-gather list
///
/// This function creates a scatter-gather list. It maps the provided pages and creates entries for the list.
///
/// # Arguments
///
/// * `pages` - A slice of virtual addresses to map.
/// * `lengths` - A slice of lengths corresponding to the pages.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<Vec<ScatterGatherEntry>, HalError>` - A result containing the scatter-gather list or an error.
pub fn create_sg_list(
    pages: &[*mut u8],
    lengths: &[usize],
    direction: DmaDirection,
) -> Result<Vec<ScatterGatherEntry>, HalError> {
    if pages.len() != lengths.len() {
        return Err(HalError::BufferError);
    }

    let mut sg_list = Vec::with_capacity(pages.len());

    for (i, (&page, &len)) in pages.iter().zip(lengths.iter()).enumerate() {
        let phys_addr = map_single(page, len, direction)?;

        sg_list.push(ScatterGatherEntry {
            addr: phys_addr,
            length: len,
            last: i == pages.len() - 1,
        });
    }

    Ok(sg_list)
}

/// Free scatter-gather list
///
/// This function frees a scatter-gather list. It unmaps the memory associated with the list.
///
/// # Arguments
///
/// * `sg_list` - A slice of scatter-gather entries to free.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn free_sg_list(sg_list: &[ScatterGatherEntry], direction: DmaDirection) -> Result<(), HalError> {
    for entry in sg_list {
        unmap_single(entry.addr, entry.length, direction)?;
    }
    Ok(())
}

/// Sync DMA memory for CPU access
///
/// This function syncs DMA memory for CPU access. It performs cache maintenance if needed.
///
/// # Arguments
///
/// * `phys_addr` - The physical address to sync.
/// * `size` - The size of the memory to sync.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn sync_single_for_cpu(
    phys_addr: usize,
    size: usize,
    direction: DmaDirection,
) -> Result<(), HalError> {
    unsafe {
        let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctx.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // Find mapping
        if let Some(mapping) = ctx.mappings.iter().find(|m| {
            m.phys_addr == phys_addr && m.size == size && m.direction == direction
        }) {
            // Perform cache maintenance if needed
            if !mapping.coherent {
                // TODO: Implement cache maintenance operations
            }
        }

        Ok(())
    }
}

/// Sync DMA memory for device access
///
/// This function syncs DMA memory for device access. It performs cache maintenance if needed.
///
/// # Arguments
///
/// * `phys_addr` - The physical address to sync.
/// * `size` - The size of the memory to sync.
/// * `direction` - The direction of the DMA transfer.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn sync_single_for_device(
    phys_addr: usize,
    size: usize,
    direction: DmaDirection,
) -> Result<(), HalError> {
    unsafe {
        let ctx = DMA_CTX.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctx.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // Find mapping
        if let Some(mapping) = ctx.mappings.iter().find(|m| {
            m.phys_addr == phys_addr && m.size == size && m.direction == direction
        }) {
            // Perform cache maintenance if needed
            if !mapping.coherent {
                // TODO: Implement cache maintenance operations
            }
        }

        Ok(())
    }
}
