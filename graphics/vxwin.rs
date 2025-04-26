// vxwin.rs

// Window rendering system module
pub mod vxwin {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXWin {
        windows: Arc<Mutex<HashMap<u32, Window>>>,
    }

    struct Window {
        id: u32,
        title: String,
        size: (u32, u32),
        position: (i32, i32),
        content: String,
    }

    impl VXWin {
        pub fn new() -> Self {
            VXWin {
                windows: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn create_window(&self, id: u32, title: String, size: (u32, u32), position: (i32, i32), content: String) {
            let mut windows = self.windows.lock().unwrap();
            windows.insert(id, Window { id, title, size, position, content });
        }

        pub fn update_window(&self, id: u32, content: String) {
            let mut windows = self.windows.lock().unwrap();
            if let Some(window) = windows.get_mut(&id) {
                window.content = content;
            }
        }

        pub fn get_window(&self, id: u32) -> Option<Window> {
            let windows = self.windows.lock().unwrap();
            windows.get(&id).cloned()
        }
    }
}
