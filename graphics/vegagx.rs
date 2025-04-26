// vegagx.rs

// Compositor, window manager, event hub module
pub mod vegagx {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VegaGX {
        windows: Arc<Mutex<HashMap<u32, Window>>>,
    }

    struct Window {
        id: u32,
        title: String,
        size: (u32, u32),
        position: (i32, i32),
    }

    impl VegaGX {
        pub fn new() -> Self {
            VegaGX {
                windows: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn create_window(&self, id: u32, title: String, size: (u32, u32), position: (i32, i32)) {
            let mut windows = self.windows.lock().unwrap();
            windows.insert(id, Window { id, title, size, position });
        }

        pub fn destroy_window(&self, id: u32) {
            let mut windows = self.windows.lock().unwrap();
            windows.remove(&id);
        }

        pub fn get_window(&self, id: u32) -> Option<Window> {
            let windows = self.windows.lock().unwrap();
            windows.get(&id).cloned()
        }
    }
}
