//! Hardware Performance Monitoring
//!
//! Provides access to CPU and device performance counters:
//! - CPU performance counters (IPC, cache misses, etc)
//! - Device performance metrics
//! - Power consumption tracking
//! Based on Intel PMU architecture

use crate::HalError;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use alloc::vec::Vec;

/// Performance counter types
///
/// This enum defines the different types of performance counters.
#[derive(Debug, Clone, Copy)]
pub enum CounterType {
    /// Instructions counter
    Instructions,
    /// Cycles counter
    Cycles,
    /// Branch misses counter
    BranchMisses,
    /// Cache misses counter
    CacheMisses,
    /// Power consumption counter
    PowerConsumption,
    /// Temperature counter
    Temperature,
}

/// Performance monitoring unit
///
/// This struct represents a performance monitoring unit.
#[derive(Debug)]
pub struct PerformanceUnit {
    /// Counter ID
    counter_id: u32,
    /// Counter type
    counter_type: CounterType,
    /// Counter value
    value: AtomicU64,
    /// Enabled flag
    enabled: AtomicBool,
}

/// PMU configuration
///
/// This struct represents the PMU configuration.
#[derive(Debug, Clone)]
pub struct PmuConfig {
    /// Sample period
    sample_period: u64,
    /// Interrupt threshold
    interrupt_threshold: u64,
    /// Counter mask
    counter_mask: u64,
}

/// Performance event selector MSR format
///
/// This struct represents the performance event selector MSR format.
#[derive(Debug)]
#[repr(C)]
struct PerfEventSelect {
    /// Event select
    event_select: u8,
    /// Unit mask
    unit_mask: u8,
    /// User mode
    user: bool,
    /// OS mode
    os: bool,
    /// Edge detect
    edge: bool,
    /// Pin control
    pc: bool,
    /// Interrupt enable
    int: bool,
    /// Enabled flag
    enabled: bool,
    /// Invert flag
    inv: bool,
    /// Counter mask
    cmask: u8,
}

/// Global PMU state
///
/// This struct represents the global PMU state.
#[derive(Debug)]
pub struct PmuManager {
    /// Initialized flag
    initialized: AtomicBool,
    /// Performance units
    units: Vec<PerformanceUnit>,
    /// PMU configuration
    config: PmuConfig,
}

// Singleton PMU manager
static mut PMU_MANAGER: Option<PmuManager> = None;

impl PmuManager {
    /// Initialize performance monitoring
    ///
    /// This function initializes the performance monitoring. It checks CPU capabilities, creates the PMU manager, and initializes the performance counters.
    pub fn init() -> Result<(), HalError> {
        unsafe {
            if PMU_MANAGER.is_some() {
                return Ok(());
            }

            // Check CPU capabilities
            if !check_pmu_support()? {
                return Err(HalError::DeviceError);
            }

            // Create PMU manager
            PMU_MANAGER = Some(PmuManager {
                initialized: AtomicBool::new(true),
                units: Vec::new(),
                config: PmuConfig {
                    sample_period: 10000,
                    interrupt_threshold: 1000,
                    counter_mask: 0,
                },
            });

            // Initialize performance counters
            init_counters()?;

            Ok(())
        }
    }

    /// Enable a performance counter
    ///
    /// This function enables a performance counter. It finds an available counter, configures it, and creates a new PMU unit.
    ///
    /// # Arguments
    ///
    /// * `counter_type` - The type of the performance counter to enable.
    ///
    /// # Returns
    ///
    /// * `Result<u32, HalError>` - A result containing the counter ID or an error.
    pub fn enable_counter(counter_type: CounterType) -> Result<u32, HalError> {
        unsafe {
            let mgr = PMU_MANAGER.as_mut().ok_or(HalError::NotInitialized)?;
            if !mgr.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Find available counter
            let counter_id = allocate_counter()?;

            // Configure counter
            configure_counter(counter_id, counter_type)?;

            // Create new PMU unit
            let unit = PerformanceUnit {
                counter_id,
                counter_type,
                value: AtomicU64::new(0),
                enabled: AtomicBool::new(true),
            };

            mgr.units.push(unit);

            Ok(counter_id)
        }
    }

    /// Read counter value
    ///
    /// This function reads the value of a performance counter. It finds the counter, reads the hardware counter value, and stores it.
    ///
    /// # Arguments
    ///
    /// * `counter_id` - The ID of the performance counter to read.
    ///
    /// # Returns
    ///
    /// * `Result<u64, HalError>` - A result containing the counter value or an error.
    pub fn read_counter(counter_id: u32) -> Result<u64, HalError> {
        unsafe {
            let mgr = PMU_MANAGER.as_ref().ok_or(HalError::NotInitialized)?;
            if !mgr.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Find counter
            if let Some(unit) = mgr.units.iter().find(|u| u.counter_id == counter_id) {
                if !unit.enabled.load(Ordering::SeqCst) {
                    return Err(HalError::DeviceError);
                }

                // Read hardware counter value
                let value = read_msr(get_counter_msr(counter_id))?;
                unit.value.store(value, Ordering::SeqCst);

                Ok(value)
            } else {
                Err(HalError::DeviceError)
            }
        }
    }

    /// Reset counter value
    ///
    /// This function resets the value of a performance counter. It finds the counter, writes zero to the counter MSR, and stores the value.
    ///
    /// # Arguments
    ///
    /// * `counter_id` - The ID of the performance counter to reset.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn reset_counter(counter_id: u32) -> Result<(), HalError> {
        unsafe {
            let mgr = PMU_MANAGER.as_ref().ok_or(HalError::NotInitialized)?;
            if !mgr.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Find counter
            if let Some(unit) = mgr.units.iter().find(|u| u.counter_id == counter_id) {
                if !unit.enabled.load(Ordering::SeqCst) {
                    return Err(HalError::DeviceError);
                }

                // Write zero to counter MSR
                write_msr(get_counter_msr(counter_id), 0)?;
                unit.value.store(0, Ordering::SeqCst);

                Ok(())
            } else {
                Err(HalError::DeviceError)
            }
        }
    }

    /// Disable counter
    ///
    /// This function disables a performance counter. It finds the counter, disables it in hardware, and removes it from the manager.
    ///
    /// # Arguments
    ///
    /// * `counter_id` - The ID of the performance counter to disable.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    pub fn disable_counter(counter_id: u32) -> Result<(), HalError> {
        unsafe {
            let mgr = PMU_MANAGER.as_mut().ok_or(HalError::NotInitialized)?;
            if !mgr.initialized.load(Ordering::SeqCst) {
                return Err(HalError::NotInitialized);
            }

            // Find and remove counter
            if let Some(pos) = mgr.units.iter().position(|u| u.counter_id == counter_id) {
                let unit = &mgr.units[pos];
                unit.enabled.store(false, Ordering::SeqCst);

                // Disable counter in hardware
                let mut select = PerfEventSelect {
                    event_select: 0,
                    unit_mask: 0,
                    user: false,
                    os: false,
                    edge: false,
                    pc: false,
                    int: false,
                    enabled: false,
                    inv: false,
                    cmask: 0,
                };
                write_msr(get_perfevtsel_msr(counter_id),
                          core::mem::transmute(select))?;

                mgr.units.remove(pos);
                Ok(())
            } else {
                Err(HalError::DeviceError)
            }
        }
    }
}

/// Check CPU PMU support
///
/// This function checks if the CPU supports PMU. It uses the CPUID instruction to check for PMU support.
///
/// # Returns
///
/// * `Result<bool, HalError>` - A result indicating whether the CPU supports PMU or an error.
unsafe fn check_pmu_support() -> Result<bool, HalError> {
    // Check CPUID for PMU support
    let mut eax: u32;
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;

    asm!(
        "cpuid",
        inout("eax") 0x0A => eax,
        out("ebx") ebx,
        out("ecx") ecx,
        out("edx") edx,
    );

    // Version ID > 0 indicates PMU support
    Ok((eax & 0xFF) > 0)
}

/// Initialize performance counters
///
/// This function initializes the performance counters. It enables the PMU globally.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
unsafe fn init_counters() -> Result<(), HalError> {
    // Enable PMU globally
    let mut val: u64;
    asm!(
        "rdmsr",
        in("ecx") 0x38F,
        out("eax") val,
    );
    val |= 1 << 0;  // Set global enable bit
    asm!(
        "wrmsr",
        in("ecx") 0x38F,
        in("eax") val,
    );

    Ok(())
}

/// Allocate a performance counter
///
/// This function allocates a performance counter. It uses a simple sequential allocation method.
///
/// # Returns
///
/// * `Result<u32, HalError>` - A result containing the counter ID or an error.
unsafe fn allocate_counter() -> Result<u32, HalError> {
    // For now, simple sequential allocation
    static mut NEXT_COUNTER: u32 = 0;
    let counter = NEXT_COUNTER;
    if counter >= 4 {  // Most CPUs have 4 counters
        return Err(HalError::DeviceError);
    }
    NEXT_COUNTER += 1;
    Ok(counter)
}

/// Configure a performance counter
///
/// This function configures a performance counter. It sets the event select, unit mask, user mode, OS mode, edge detect, pin control, interrupt enable, enabled flag, invert flag, and counter mask.
///
/// # Arguments
///
/// * `counter_id` - The ID of the performance counter to configure.
/// * `counter_type` - The type of the performance counter to configure.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
unsafe fn configure_counter(counter_id: u32, counter_type: CounterType) -> Result<(), HalError> {
    let select = PerfEventSelect {
        event_select: match counter_type {
            CounterType::Instructions => 0xC0,
            CounterType::Cycles => 0x3C,
            CounterType::BranchMisses => 0xC5,
            CounterType::CacheMisses => 0x2E,
            CounterType::PowerConsumption => 0xA0,
            CounterType::Temperature => 0xA1,
        },
        unit_mask: 0x00,
        user: true,
        os: true,
        edge: false,
        pc: false,
        int: true,
        enabled: true,
        inv: false,
        cmask: 0,
    };

    write_msr(get_perfevtsel_msr(counter_id),
              core::mem::transmute(select))?;

    Ok(())
}

/// Read MSR value
///
/// This function reads the value of an MSR. It uses the RDMSR instruction to read the MSR value.
///
/// # Arguments
///
/// * `msr` - The address of the MSR to read.
///
/// # Returns
///
/// * `Result<u64, HalError>` - A result containing the MSR value or an error.
unsafe fn read_msr(msr: u32) -> Result<u64, HalError> {
    let mut value: u64;
    asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") value,
    );
    Ok(value)
}

/// Write MSR value
///
/// This function writes a value to an MSR. It uses the WRMSR instruction to write the MSR value.
///
/// # Arguments
///
/// * `msr` - The address of the MSR to write.
/// * `value` - The value to write to the MSR.
///
/// # Returns
///
/// * `Result<(), HalError>` - A result indicating success or an error.
unsafe fn write_msr(msr: u32, value: u64) -> Result<(), HalError> {
    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") value,
    );
    Ok(())
}

/// Get performance counter MSR address
///
/// This function returns the MSR address of a performance counter.
///
/// # Arguments
///
/// * `counter_id` - The ID of the performance counter.
///
/// # Returns
///
/// * `u32` - The MSR address of the performance counter.
const fn get_counter_msr(counter_id: u32) -> u32 {
    0xC1 + counter_id  // IA32_PMCx MSRs
}

/// Get performance event select MSR address
///
/// This function returns the MSR address of a performance event selector.
///
/// # Arguments
///
/// * `counter_id` - The ID of the performance event selector.
///
/// # Returns
///
/// * `u32` - The MSR address of the performance event selector.
const fn get_perfevtsel_msr(counter_id: u32) -> u32 {
    0x186 + counter_id  // IA32_PERFEVTSELx MSRs
}
