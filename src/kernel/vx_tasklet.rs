/// Micro-threading and task scheduling for VaelixCore.
/// This module provides basic task scheduling and micro-threading functions.
pub mod vx_tasklet {
    use core::sync::atomic::{AtomicUsize, Ordering};

    static TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

    pub fn init() { // Added pub
        // Initialize task scheduling
        println!("Initializing task scheduling...");
    }

    pub fn process_events() { // Added pub
        // Process task events
        println!("Processing task events...");
    }

    pub fn create_task() -> usize {
        let task_id = TASK_COUNT.fetch_add(1, Ordering::SeqCst);
        println!("Created task with ID: {}", task_id);
        task_id
    }
}
