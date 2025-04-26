// vxchan.rs

// Named IPC channels module
pub mod vxchan {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXChan {
        channels: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    }

    impl VXChan {
        pub fn new() -> Self {
            VXChan {
                channels: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn send(&self, channel: &str, message: &[u8]) {
            let mut channels = self.channels.lock().unwrap();
            channels.entry(channel.to_string()).or_insert_with(Vec::new).extend_from_slice(message);
        }

        pub fn receive(&self, channel: &str) -> Option<Vec<u8>> {
            let mut channels = self.channels.lock().unwrap();
            channels.get_mut(channel).map(|msg| {
                let mut message = msg.clone();
                message.clear();
                message
            })
        }
    }
}
