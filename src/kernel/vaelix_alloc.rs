use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::ptr::null_mut;
use std::alloc::System;

struct VaelixAllocator;

unsafe impl GlobalAlloc for VaelixAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Implement memory allocation with failure detection
        let ptr = System.alloc(layout);
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Implement memory deallocation
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: VaelixAllocator = VaelixAllocator;
