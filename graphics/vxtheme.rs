// vxtheme.rs

// Custom theming engine module
pub mod vxtheme {
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize)]
    struct Theme {
        primary_color: String,
        secondary_color: String,
        font_family: String,
    }

    struct VXTheme {
        theme: Arc<Mutex<Theme>>,
    }

    impl VXTheme {
        pub fn new() -> Self {
            VXTheme {
                theme: Arc::new(Mutex::new(Theme {
                    primary_color: String::from("#007bff"),
                    secondary_color: String::from("#6c757d"),
                    font_family: String::from("Arial, sans-serif"),
                })),
            }
        }

        pub fn load_theme(&self, path: &Path) -> std::io::Result<()> {
            let mut file = fs::File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let theme: Theme = serde_json::from_str(&contents)?;
            let mut theme_lock = self.theme.lock().unwrap();
            *theme_lock = theme;
            Ok(())
        }

        pub fn get_theme(&self) -> Theme {
            let theme = self.theme.lock().unwrap();
            theme.clone()
        }
    }
}
