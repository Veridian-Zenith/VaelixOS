// src/kernel/mod.rs

pub mod vaelix_alloc;
pub mod vx_tasklet;
pub mod vxboot;
pub mod vxchan;
pub mod vxfs;
pub mod vxshield;

pub use vx_tasklet::vx_tasklet_init;
pub use vxchan::vxchan::vxchan_init;
