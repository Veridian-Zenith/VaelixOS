// vxfont.rs

// Font system handling scalable rendering module
pub mod vxfont {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    struct VXFont {
        fonts: Arc<Mutex<HashMap<String, Font>>>,
    }

    struct Font {
        name: String,
        size: u32,
        data: Vec<u8>,
    }

    impl VXFont {
        pub fn new() -> Self {
            VXFont {
                fonts: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn load_font(&self, name: String, size: u32, data: Vec<u8>) {
            let mut fonts = self.fonts.lock().unwrap();
            fonts.insert(name, Font { name, size, data });
        }

        pub fn get_font(&self, name: &str) -> Option<Font> {
            let fonts = self.fonts.lock().unwrap();
            fonts.get(name).cloned()
        }
    }
}
