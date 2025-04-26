// vxtoml.rs

// Manifest file format module
pub mod vxtoml {
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use toml;

    pub fn parse_toml(path: &Path) -> Result<toml::Value, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let parsed = toml::from_str(&contents)?;
        Ok(parsed)
    }
}
