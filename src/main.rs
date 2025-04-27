mod kernel;


fn main() -> ! {
    println!("Welcome to VaelixOS!");

    // Initialize the kernel's core components
            kernel::vx_tasklet_init();
    kernel::vxchan_init();

    println!("VaelixCore initialized.");

    loop {} // Kernel main loop
}
