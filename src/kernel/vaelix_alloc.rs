// vaelix_alloc.rs

// Custom Rust memory management module
pub mod vaelix_alloc {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct VaelixAllocator {
        allocated: AtomicUsize,
        deallocated: AtomicUsize,
    }

    unsafe impl GlobalAlloc for VaelixAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = System.alloc(layout);
            if !ptr.is_null() {
                self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
            }
            ptr
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            self.deallocated.fetch_add(layout.size(), Ordering::Relaxed);
            System.dealloc(ptr, layout);
        }
    }

    static ALLOCATOR: VaelixAllocator = VaelixAllocator {
        allocated: AtomicUsize::new(0),
        deallocated: AtomicUsize::new(0),
    };

    #[global_allocator]
    static GLOBAL: VaelixAllocator = &ALLOCATOR;

    pub fn init() {
        // Initialize the allocator
    }

    pub fn stats() -> (usize, usize) {
        (ALLOCATOR.allocated.load(Ordering::Relaxed), ALLOCATOR.deallocated.load(Ordering::Relaxed))
    }
}
