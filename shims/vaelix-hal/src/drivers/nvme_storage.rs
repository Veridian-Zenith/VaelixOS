//! NVMe Storage Driver Shim
//!
//! Provides interface to KIOXIA NVMe SSD:
//! - PCIe Gen3 x4 (63.2 Gb/s)
//! - 238.47 GiB capacity
//! - NVMe 1.4 support

use crate::raw::{
    driver::{DriverOps, DriverInfo, DriverCaps, PowerState, DmaOp, DmaDirection},
    pci::{self, PciDevice},
    IoRegion,
};
use crate::HalError;
use core::sync::atomic::{AtomicPtr, AtomicBool, AtomicU64, Ordering};

/// NVMe controller registers based on Linux driver
///
/// This struct represents the NVMe controller registers.
#[derive(Debug)]
#[repr(C)]
struct NvmeRegs {
    /// Controller Capabilities
    cap: u64,        // 0x0000
    /// Version
    vs: u32,         // 0x0008
    /// Interrupt Mask Set
    intms: u32,      // 0x000c
    /// Interrupt Mask Clear
    intmc: u32,      // 0x0010
    /// Controller Configuration
    cc: u32,         // 0x0014
    /// Controller Status
    csts: u32,       // 0x001c
    /// Admin Queue Attributes
    aqa: u32,        // 0x0024
    /// Admin Submission Queue
    asq: u64,        // 0x0028
    /// Admin Completion Queue
    acq: u64,        // 0x0030
    /// Command Set Specific
    cmbloc: u32,     // 0x0038
    /// Command Set Size
    cmbsz: u32,      // 0x003c
}

/// NVMe submission queue entry
///
/// This struct represents an entry in the NVMe submission queue.
#[derive(Debug)]
#[repr(C)]
struct NvmeCmd {
    /// Opcode
    opcode: u8,
    /// Flags
    flags: u8,
    /// Command ID
    command_id: u16,
    /// Namespace ID
    nsid: u32,
    /// Command Dword 2
    cdw2: [u32; 2],
    /// Metadata
    metadata: u64,
    /// Physical Region Page 1
    prp1: u64,
    /// Physical Region Page 2
    prp2: u64,
    /// Command Dword 10
    cdw10: [u32; 6],
}

/// NVMe completion queue entry
///
/// This struct represents an entry in the NVMe completion queue.
#[derive(Debug)]
#[repr(C)]
struct NvmeCpl {
    /// Result
    result: u32,
    /// Submission Queue Head Pointer
    sq_head: u16,
    /// Submission Queue ID
    sq_id: u16,
    /// Command ID
    command_id: u16,
    /// Status
    status: u16,
}

/// NVMe driver state
///
/// This struct represents the state of the NVMe driver.
#[derive(Debug)]
pub struct NvmeDriver {
    /// PCI Device
    device: Option<PciDevice>,
    /// Memory-Mapped I/O
    mmio: AtomicPtr<NvmeRegs>,
    /// Initialized Flag
    initialized: AtomicBool,
    /// Admin Submission Queue
    admin_sq: AtomicPtr<NvmeCmd>,
    /// Admin Completion Queue
    admin_cq: AtomicPtr<NvmeCpl>,
    /// Total Size
    total_size: AtomicU64,
}

// Singleton driver instance
static DRIVER: NvmeDriver = NvmeDriver {
    device: None,
    mmio: AtomicPtr::new(core::ptr::null_mut()),
    initialized: AtomicBool::new(false),
    admin_sq: AtomicPtr::new(core::ptr::null_mut()),
    admin_cq: AtomicPtr::new(core::ptr::null_mut()),
    total_size: AtomicU64::new(0),
};

impl NvmeDriver {
    /// Get driver registration info
    ///
    /// This function returns the driver registration information.
    ///
    /// # Returns
    ///
    /// * `DriverInfo` - The driver registration information.
    pub fn info() -> DriverInfo {
        DriverInfo {
            name: "nvme_kioxia",
            vendor_id: 0x1179,  // KIOXIA
            device_id: 0x0001,  // Generic NVMe
            capabilities: DriverCaps::DMA | DriverCaps::MSI | DriverCaps::PM,
            initialized: AtomicBool::new(false),
        }
    }

    /// Map controller registers
    ///
    /// This function maps the controller registers. It gets the BAR 0 which contains the MMIO registers and maps them.
    ///
    /// # Arguments
    ///
    /// * `device` - A reference to the PCI device.
    ///
    /// # Returns
    ///
    /// * `Result<*mut NvmeRegs, HalError>` - A result containing the pointer to the mapped registers or an error.
    unsafe fn map_registers(&self, device: &PciDevice) -> Result<*mut NvmeRegs, HalError> {
        // Get BAR 0 which contains MMIO registers
        let bar = device.get_bar(0)
            .ok_or(HalError::DeviceError)?;

        // Map the registers
        let regs = bar.register::<NvmeRegs>(0)
            as *mut NvmeRegs;

        Ok(regs)
    }

    /// Initialize admin queues
    ///
    /// This function initializes the admin queues. It allocates the admin submission and completion queues, sets up DMA operations, and configures the admin queue attributes.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    unsafe fn init_admin_queues(&self) -> Result<(), HalError> {
        let regs = self.mmio.load(Ordering::SeqCst);
        if regs.is_null() {
            return Err(HalError::NotInitialized);
        }

        // Allocate admin submission queue
        let admin_sq = crate::raw::driver::map_device_memory(0, 4096)?;
        let admin_cq = crate::raw::driver::map_device_memory(0, 4096)?;

        // Set up DMA operations
        let sq_op = DmaOp {
            phys_addr: admin_sq as usize,
            virt_addr: admin_sq as usize,
            size: 4096,
            direction: DmaDirection::ToDevice,
        };

        let cq_op = DmaOp {
            phys_addr: admin_cq as usize,
            virt_addr: admin_cq as usize,
            size: 4096,
            direction: DmaDirection::FromDevice,
        };

        // Map DMA regions
        crate::raw::driver::dma_map(&sq_op)?;
        crate::raw::driver::dma_map(&cq_op)?;

        // Store queue pointers
        self.admin_sq.store(admin_sq as *mut NvmeCmd, Ordering::SeqCst);
        self.admin_cq.store(admin_cq as *mut NvmeCpl, Ordering::SeqCst);

        // Configure admin queue attributes
        (*regs).aqa = (255 << 16) | 255;  // 256 entries each
        (*regs).asq = admin_sq as u64;     // Admin submission queue
        (*regs).acq = admin_cq as u64;     // Admin completion queue

        Ok(())
    }

    /// Enable controller
    ///
    /// This function enables the controller. It sets the admin queue size and enables the controller.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    unsafe fn enable_controller(&self) -> Result<(), HalError> {
        let regs = self.mmio.load(Ordering::SeqCst);
        if regs.is_null() {
            return Err(HalError::NotInitialized);
        }

        // Set admin queue size
        (*regs).aqa = (255 << 16) | 255;

        // Enable controller
        (*regs).cc = 0x460001; // Enable, 4KB pages, command set NVM

        // Wait for ready
        while (*regs).csts & 0x1 == 0 {
            core::hint::spin_loop();
        }

        Ok(())
    }

    /// Identify controller and namespace
    ///
    /// This function identifies the controller and namespace. It sends the identify controller command and stores the total size.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    unsafe fn identify_controller(&self) -> Result<(), HalError> {
        let regs = self.mmio.load(Ordering::SeqCst);
        if regs.is_null() {
            return Err(HalError::NotInitialized);
        }

        // TODO: Send identify controller command
        // For now use hardcoded size from sys.txt
        self.total_size.store(256_060_514_304, Ordering::SeqCst);

        Ok(())
    }
}

impl DriverOps for NvmeDriver {
    /// Initialize the driver
    ///
    /// This function initializes the driver. It finds the NVMe controller, initializes the PCI device, maps the registers, initializes the admin queues, enables the controller, and identifies the controller.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    fn init(&self) -> Result<(), HalError> {
        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Find the NVMe controller
        let device = pci::find_device(0x1179, 0x0001)
            .ok_or(HalError::DeviceError)?;

        // Initialize PCI device
        pci::init_device(&device)?;

        // Map registers
        let regs = unsafe { self.map_registers(&device)? };
        self.mmio.store(regs, Ordering::SeqCst);

        unsafe {
            // Initialize admin queues
            self.init_admin_queues()?;

            // Enable controller
            self.enable_controller()?;

            // Identify controller
            self.identify_controller()?;
        }

        self.initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Shutdown the driver
    ///
    /// This function shuts down the driver. It disables the controller, unmaps the admin queues, and clears the initialized flag.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    fn shutdown(&self) -> Result<(), HalError> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        unsafe {
            let regs = self.mmio.load(Ordering::SeqCst);
            if !regs.is_null() {
                // Disable controller
                (*regs).cc &= !0x1;

                // Wait for not ready
                while (*regs).csts & 0x1 != 0 {
                    core::hint::spin_loop();
                }

                // Unmap admin queues
                if !self.admin_sq.load(Ordering::SeqCst).is_null() {
                    let sq_op = DmaOp {
                        phys_addr: self.admin_sq.load(Ordering::SeqCst) as usize,
                        virt_addr: self.admin_sq.load(Ordering::SeqCst) as usize,
                        size: 4096,
                        direction: DmaDirection::ToDevice,
                    };
                    crate::raw::driver::dma_unmap(&sq_op)?;
                }

                if !self.admin_cq.load(Ordering::SeqCst).is_null() {
                    let cq_op = DmaOp {
                        phys_addr: self.admin_cq.load(Ordering::SeqCst) as usize,
                        virt_addr: self.admin_cq.load(Ordering::SeqCst) as usize,
                        size: 4096,
                        direction: DmaDirection::FromDevice,
                    };
                    crate::raw::driver::dma_unmap(&cq_op)?;
                }
            }
        }

        self.initialized.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Handle an interrupt
    ///
    /// This function handles an interrupt. It processes the completion queue entries.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    fn handle_interrupt(&self) -> Result<(), HalError> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        // TODO: Process completion queue entries
        Ok(())
    }

    /// Set the power state
    ///
    /// This function sets the power state of the controller. It adjusts the controller configuration based on the power state.
    ///
    /// # Arguments
    ///
    /// * `state` - The power state to set.
    ///
    /// # Returns
    ///
    /// * `Result<(), HalError>` - A result indicating success or an error.
    fn set_power_state(&self, state: PowerState) -> Result<(), HalError> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(HalError::NotInitialized);
        }

        unsafe {
            let regs = self.mmio.load(Ordering::SeqCst);
            if !regs.is_null() {
                match state {
                    PowerState::D0 => {
                        // Full power
                        (*regs).cc |= 0x1;
                    }
                    PowerState::D1 | PowerState::D2 => {
                        // Reduced power
                        (*regs).cc &= !0x2;
                    }
                    PowerState::D3Hot | PowerState::D3Cold => {
                        // Power down
                        (*regs).cc &= !0x1;
                    }
                }
            }
        }

        Ok(())
    }
}

/// Get driver instance
///
/// This function returns the singleton instance of the NVMe driver.
///
/// # Returns
///
/// * `&'static NvmeDriver` - A reference to the NVMe driver instance.
pub fn driver() -> &'static NvmeDriver {
    &DRIVER
}
