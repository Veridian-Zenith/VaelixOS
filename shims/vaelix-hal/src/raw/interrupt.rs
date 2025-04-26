//! Interrupt Management
//!
//! Provides interrupt handling infrastructure:
//! - MSI/MSI-X support
//! - IRQ registration and dispatch
//! - Interrupt thread management

use crate::HalError;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use alloc::collections::BTreeMap;
use alloc::boxed::Box;

/// Interrupt handler function type
///
/// This type represents the function type for an interrupt handler.
#[derive(Debug)]
type IrqHandler = Box<dyn Fn() -> Result<(), HalError> + Send + Sync>;

/// Interrupt types
///
/// This enum defines the different types of interrupts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptType {
    /// Legacy interrupt type
    Legacy,
    /// MSI interrupt type
    Msi,
    /// MSI-X interrupt type
    MsiX,
}

/// Interrupt controller state
///
/// This struct represents the state of the interrupt controller.
#[derive(Debug)]
struct InterruptController {
    /// Enabled flag
    enabled: AtomicBool,
    /// Current vector
    current_vector: AtomicU32,
    /// Handlers map
    handlers: BTreeMap<u32, IrqHandler>,
}

// Global interrupt controller
static mut INTERRUPT_CTRL: Option<InterruptController> = None;

/// Initialize interrupt subsystem
///
/// This function initializes the interrupt subsystem. It enables the APIC for modern interrupt handling.
pub fn init() -> Result<(), HalError> {
    unsafe {
        if INTERRUPT_CTRL.is_some() {
            return Ok(());
        }

        INTERRUPT_CTRL = Some(InterruptController {
            enabled: AtomicBool::new(false),
            current_vector: AtomicU32::new(32), // Start after CPU exceptions
            handlers: BTreeMap::new(),
        });

        // Enable APIC for modern interrupt handling
        enable_apic()?;

        Ok(())
    }
}

/// Enable Advanced Programmable Interrupt Controller
///
/// This function enables the APIC by setting bit 11 in MSR 0x1B and initializing the APIC base.
fn enable_apic() -> Result<(), HalError> {
    unsafe {
        // Enable APIC by setting bit 11 in MSR 0x1B
        let mut msr = 0u64;
        asm!("rdmsr", in("ecx") 0x1B, out("eax") msr);
        msr |= 1 << 11;
        asm!("wrmsr", in("ecx") 0x1B, in("eax") msr);

        // Initialize APIC base
        asm!("wrmsr", in("ecx") 0x1B, in("eax") 0xFEE00000, in("edx") 0);

        Ok(())
    }
}

/// Register an interrupt handler
///
/// This function registers an interrupt handler. It stores the handler in the interrupt controller.
///
/// # Arguments
///
/// * `irq` - The interrupt request number.
/// * `handler` - The interrupt handler function.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result containing the interrupt handler or an error.
pub fn register_handler(
    irq: u32,
    handler: Box<dyn Fn() -> Result<(), HalError> + Send + Sync>,
) -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Store handler
        ctrl.handlers.insert(irq, handler);

        Ok(())
    }
}

/// Unregister an interrupt handler
///
/// This function unregisters an interrupt handler. It removes the handler from the interrupt controller.
///
/// # Arguments
///
/// * `irq` - The interrupt request number.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn unregister_handler(irq: u32) -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Remove handler
        ctrl.handlers.remove(&irq);

        Ok(())
    }
}

/// Enable interrupts globally
///
/// This function enables interrupts globally. It sets the enabled flag and enables interrupts using the STI instruction.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn enable() -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Enable interrupts
        asm!("sti");
        ctrl.enabled.store(true, Ordering::SeqCst);

        Ok(())
    }
}

/// Disable interrupts globally
///
/// This function disables interrupts globally. It clears the enabled flag and disables interrupts using the CLI instruction.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn disable() -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Disable interrupts
        asm!("cli");
        ctrl.enabled.store(false, Ordering::SeqCst);

        Ok(())
    }
}

/// Handle an interrupt
///
/// This function handles an interrupt. It finds and calls the corresponding handler and sends an EOI if necessary.
///
/// # Arguments
///
/// * `vector` - The interrupt vector.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn handle_interrupt(vector: u32) -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_ref().ok_or(HalError::NotInitialized)?;

        // Find and call handler
        if let Some(handler) = ctrl.handlers.get(&vector) {
            handler()?;
        }

        // Send EOI if not MSI/MSI-X
        if vector < 32 {
            send_eoi(vector);
        }

        Ok(())
    }
}

/// Send End-Of-Interrupt signal
///
/// This function sends an End-Of-Interrupt signal. It writes to the APIC EOI register.
///
/// # Arguments
///
/// * `vector` - The interrupt vector.
fn send_eoi(vector: u32) {
    unsafe {
        // Write to APIC EOI register
        core::ptr::write_volatile(0xFEE00000 as *mut u32, 0);
    }
}

/// Allocate an MSI vector
///
/// This function allocates an MSI vector. It returns the next available vector.
///
/// # Returns
///
/// * `Result<u32, HalError>` - A result containing the MSI vector or an error.
pub fn allocate_msi_vector() -> Result<u32, HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Allocate next available vector
        let vector = ctrl.current_vector.fetch_add(1, Ordering::SeqCst);
        if vector >= 256 {
            return Err(HalError::DeviceError);
        }

        Ok(vector)
    }
}

/// Free an MSI vector
///
/// This function frees an MSI vector. It removes the handler associated with the vector.
///
/// # Arguments
///
/// * `vector` - The MSI vector to free.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn free_msi_vector(vector: u32) -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;

        // Simple implementation - just remove handler
        ctrl.handlers.remove(&vector);

        Ok(())
    }
}

/// Configure MSI for a device
///
/// This function configures MSI for a device. It writes the MSI address and data to the device config space.
///
/// # Arguments
///
/// * `address` - The MSI address.
/// * `data` - The MSI data.
/// * `vector` - The MSI vector.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
pub fn configure_msi(
    address: u64,
    data: u32,
    vector: u32,
) -> Result<(), HalError> {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_mut().ok_or(HalError::NotInitialized)?;
        if !ctrl.enabled.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // Write MSI address to device config space
        // TODO: Implement PCI config space writes

        // Write MSI data
        // TODO: Implement PCI config space writes

        Ok(())
    }
}

/// Check if interrupts are enabled
///
/// This function checks if interrupts are enabled. It returns the enabled flag.
///
/// # Returns
///
/// * `bool` - A boolean indicating whether interrupts are enabled.
pub fn are_enabled() -> bool {
    unsafe {
        let ctrl = INTERRUPT_CTRL.as_ref();
        ctrl.map_or(false, |c| c.enabled.load(Ordering::SeqCst))
    }
}

/// Interrupt guard scope for critical sections
///
/// This struct represents an interrupt guard scope for critical sections.
#[derive(Debug)]
pub struct InterruptGuard {
    /// Was enabled flag
    was_enabled: bool,
}

impl InterruptGuard {
    /// Create new interrupt guard
    ///
    /// This function creates a new interrupt guard. It disables interrupts if they were previously enabled.
    ///
    /// # Returns
    ///
    /// * `Result<Self, HalError>` - A result containing the interrupt guard or an error.
    pub fn new() -> Result<Self, HalError> {
        let was_enabled = are_enabled();
        if was_enabled {
            disable()?;
        }
        Ok(Self { was_enabled })
    }
}

impl Drop for InterruptGuard {
    /// Drop implementation
    ///
    /// This function is called when the interrupt guard goes out of scope. It re-enables interrupts if they were previously enabled.
    fn drop(&mut self) {
        if self.was_enabled {
            let _ = enable();
        }
    }
}
