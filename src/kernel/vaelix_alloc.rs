/// Basic memory allocation for VaelixCore.
/// This module provides simple memory allocation functions.
pub mod vaelix_alloc {
    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;
    use core::ptr;
    use core::sync::atomic::{AtomicUsize, Ordering};

    struct VaelixAllocator {
        heap_start: UnsafeCell<*mut u8>,
        heap_end: UnsafeCell<*mut u8>,
        next: AtomicUsize,
    }

    unsafe impl Sync for VaelixAllocator {}

    unsafe impl GlobalAlloc for VaelixAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let size = layout.size();
            let _align = layout.align();
            let current_next = self.next.load(Ordering::Relaxed);
            let new_next = current_next + size;

            if new_next > *self.heap_end.get() as usize {
                ptr::null_mut() // Out of memory
            } else {
                self.next.store(new_next, Ordering::Relaxed);
                (*self.heap_start.get() as usize + current_next) as *mut u8
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
            // Deallocation logic will be implemented later.
            // For now, we are only focusing on basic allocation.
        }
    }

    impl VaelixAllocator {
        /// Initializes the Vaelix allocator with a given heap start address and size.
        pub fn init(&self, heap_start: *mut u8, heap_size: usize) {
            unsafe {
                *self.heap_start.get() = heap_start;
                *self.heap_end.get() = heap_start.add(heap_size);
                self.next.store(0, Ordering::Relaxed);
            }
        }
    }

    /// The global allocator instance used by the kernel.
    #[global_allocator]
    static GLOBAL: VaelixAllocator = VaelixAllocator {
        heap_start: UnsafeCell::new(core::ptr::null_mut()),
        heap_end: UnsafeCell::new(core::ptr::null_mut()),
        next: AtomicUsize::new(0),
    };

    /// Provides a C-compatible interface for allocating memory.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the returned pointer is eventually freed using `vaelix_free`.
    #[no_mangle]
    pub extern "C" fn vaelix_alloc(size: usize) -> *mut u8 {
        unsafe { GLOBAL.alloc(Layout::from_size_align(size, core::mem::align_of::<usize>()).unwrap()) }
    }

    /// Provides a C-compatible interface for deallocating memory.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` was previously allocated by `vaelix_alloc`.
    #[no_mangle]
    pub extern "C" fn vaelix_free(ptr: *mut u8) {
        unsafe { GLOBAL.dealloc(ptr, Layout::from_size_align(core::mem::size_of::<usize>(), core::mem::align_of::<usize>()).unwrap()) }
    }

    /// Initializes the global Vaelix allocator. This function should be called once
    /// during kernel initialization with the start address and size of the kernel heap.
    pub fn init_global(_heap_start: *mut u8, _heap_size: usize) {
    }
}
