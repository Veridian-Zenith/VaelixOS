#[cfg(test)]
mod tests {
    use crate::kernel::{vaelix_alloc_init, vx_tasklet_init, vxchan_init};

    #[test]
    fn test_vaelix_alloc_init() {
        let heap_start = 0x40000000 as *mut u8;
        let heap_size = 1024 * 1024;
        vaelix_alloc_init(heap_start, heap_size);
        // Add assertions to verify the initialization
    }

    #[test]
    fn test_vx_tasklet_init() {
        vx_tasklet_init();
        // Add assertions to verify the initialization
    }

    #[test]
    fn test_vxchan_init() {
        vxchan_init();
        // Add assertions to verify the initialization
    }
}
