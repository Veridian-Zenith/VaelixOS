#[cfg(test)]
pub mod tests {
    use vaelix_core::{vx_tasklet_init, vxchan_init};


    #[test]
    pub fn test_vx_tasklet_init() {
        vx_tasklet_init();
        // Add assertions to verify the initialization
    }

    #[test]
    pub fn test_vxchan_init() {
        vxchan_init();
        // Add assertions to verify the initialization
    }
}
