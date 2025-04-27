// src/kernel/mod.rs

pub mod vaelix_alloc;
pub mod vx_tasklet;
pub mod vxboot;
pub mod vxchan;
pub use vxchan::vxchan_init;
pub mod alloc;
pub mod collections;
pub mod fs;
pub mod io;
pub mod path;
pub mod process;
pub mod sync;
pub mod time;
pub mod vxfs;
pub mod vxshield;

pub use vx_tasklet::vx_tasklet_init;
