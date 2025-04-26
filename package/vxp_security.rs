// vxp_security.rs

// SHA256 checksum enforcement module
pub mod vxp_security {
    use sha2::{Sha256, Digest};
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    pub fn calculate_checksum(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    pub fn verify_checksum(file_path: &Path, expected_checksum: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let calculated_checksum = calculate_checksum(file_path)?;
        Ok(calculated_checksum == expected_checksum)
    }
}
