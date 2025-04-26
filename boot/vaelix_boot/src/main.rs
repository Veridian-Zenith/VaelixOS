#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(abi_x86_interrupt)] // Enable x86-interrupt ABI

use bootloader::{BootConfig, BootInfo};
use core::panic::PanicInfo;
use x86_64::{
    structures::paging::{PageTable, PageTableFlags},
    VirtAddr,
};

/// VaelixBoot Configuration
///
/// This constant defines the configuration settings for the VaelixBoot bootloader.
/// It includes settings such as the kernel stack size and the physical memory offset.
const BOOT_CONFIG: BootConfig = {
    let mut config = BootConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100KB stack
    config.physical_memory_offset = 0xFFFF800000000000;
    config
};

/// Entry point for the bootloader
///
/// This function is the entry point for the VaelixBoot bootloader. It is called by the bootloader crate
/// after basic CPU initialization. This function initializes the logger, sets up memory mapping, loads
/// kernel modules, and transfers control to the kernel.
///
/// # Arguments
///
/// * `boot_info` - A mutable reference to the boot information structure.
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function is safe to call as it performs necessary initialization steps.
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static mut BootInfo) -> ! {
    // Initialize our custom logger for debugging
    init_logger();

    // Print welcome message
    log!("VaelixBoot v0.1.0 starting...");

    // Set up identity mapping for the first 1GB of memory
    setup_memory_mapping(boot_info);

    // Load kernel modules
    load_kernel_modules(boot_info);

    // Transfer control to the kernel
    jump_to_kernel(boot_info);

    // Should never reach here
    loop {}
}

/// Initialize basic logging functionality
///
/// This function initializes the custom logger used for debugging purposes. The implementation
/// will be added later.
fn init_logger() {
    // Implementation will be added later
}

/// Set up initial memory mapping
///
/// This function sets up the initial memory mapping for the system. It creates initial page table entries
/// and maps the first 1GB of memory identity mapped. The detailed implementation will be added later.
///
/// # Arguments
///
/// * `boot_info` - A reference to the boot information structure.
fn setup_memory_mapping(boot_info: &'static BootInfo) {
    let phys_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // Create initial page table entries
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    // Map the first 1GB identity mapped
    // Detailed implementation will be added
}

/// Load necessary kernel modules
///
/// This function loads the necessary kernel modules into memory. The implementation will involve scanning
/// for modules in designated locations, loading them into memory, and setting up appropriate mappings.
///
/// # Arguments
///
/// * `boot_info` - A reference to the boot information structure.
fn load_kernel_modules(boot_info: &'static BootInfo) {
    // Implementation for loading modules will be added
    // This will involve:
    // 1. Scanning for modules in designated locations
    // 2. Loading them into memory
    // 3. Setting up appropriate mappings
}

/// Transfer control to the kernel
///
/// This function transfers control to the kernel. The implementation for the kernel handoff will be added
/// later. Currently, it contains a temporary loop until the implementation is complete.
///
/// # Arguments
///
/// * `boot_info` - A reference to the boot information structure.
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function is safe to call as it performs the necessary steps to transfer control to the kernel.
fn jump_to_kernel(boot_info: &'static BootInfo) -> ! {
    // Implementation for kernel handoff will be added
    loop {} // Temporary loop until implementation
}

/// This function is called on panic
///
/// This function is the panic handler for the bootloader. It is called when a panic occurs. Currently,
/// it contains an infinite loop.
///
/// # Arguments
///
/// * `info` - A reference to the panic information structure.
///
/// # Panics
///
/// This function does not panic.
///
/// # Safety
///
/// This function is safe to call as it handles panics by entering an infinite loop.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
