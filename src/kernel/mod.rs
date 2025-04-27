pub mod vaelix_alloc;
pub mod vx_tasklet;
pub mod vxchan;
pub mod vxfs;
pub mod vxboot;

pub use vaelix_alloc::init as vaelix_alloc_init;
pub use vx_tasklet::init as vx_tasklet_init;
pub use vx_tasklet::process_events as vx_tasklet_process_events;
pub use vxchan::init as vxchan_init;
pub use vxfs::init as vxfs_init;
pub use vxboot::init as vxboot_init;
