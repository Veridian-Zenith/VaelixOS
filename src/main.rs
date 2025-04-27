mod kernel;

const KERNEL_HEAP_START: usize = 0x40000000; // Example start address
const KERNEL_HEAP_SIZE: usize = 1024 * 1024;   // Example size (1MB)

fn main() -> ! {
    println!("Welcome to VaelixOS!");

    // Initialize the kernel's core components
    let heap_start = KERNEL_HEAP_START as *mut u8;
    kernel::vaelix_alloc_init(heap_start, KERNEL_HEAP_SIZE);
    kernel::vx_tasklet_init();
    kernel::vxchan_init();

    println!("VaelixCore initialized.");

    loop {} // Kernel main loop
}
