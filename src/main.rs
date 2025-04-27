fn main() {
    use vaelix_core::vx_tasklet::vx_tasklet_init;
    use vaelix_core::vxchan::vxchan::vxchan_init;
    use vaelix_core::vxboot::vxboot::boot;

    // Initialize the tasklet scheduler
    let scheduler = vx_tasklet_init();

    // Initialize the VXChan module
    let vxchan_manager = vxchan_init().expect("Failed to initialize VXChan");

    // Start the boot process
    boot().expect("Failed to boot the system");

    loop {} // Kernel main loop
}
