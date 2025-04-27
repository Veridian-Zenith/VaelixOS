// src/kernel/mod.rs

pub mod vx_tasklet;
pub mod vxboot;
pub mod vaelix_alloc;
pub mod vxchan;
pub mod vxfs;
pub mod sync;
pub mod collections;
pub mod fs;
pub mod io;
pub mod path;
pub mod time;
pub mod process;
pub mod alloc;

pub use vx_tasklet::vx_tasklet_init;
pub use vxchan::vxchan_init;
