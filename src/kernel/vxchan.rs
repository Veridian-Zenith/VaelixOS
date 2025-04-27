pub mod vxchan {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{self, Sender, Receiver};
    

    pub struct VXChan {
        sender: Sender<String>,
        receiver: Receiver<String>,
    }

    impl VXChan {
        pub fn new() -> VXChan {
            let (sender, receiver) = mpsc::channel();
            VXChan { sender, receiver }
        }

        pub fn send(&self, message: String) -> Result<(), &'static str> {
            self.sender.send(message).map_err(|_| "Failed to send message")
        }

        pub fn receive(&self) -> Result<String, &'static str> {
            self.receiver.recv().map_err(|_| "Failed to receive message")
        }
    }

    pub struct VXChanManager {
        channels: Arc<Mutex<HashMap<String, Arc<Mutex<VXChan>>>>>,
    }

    impl VXChanManager {
        pub fn new() -> Self {
            VXChanManager {
                channels: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn create_channel(&self, name: &str) -> Result<(), &'static str> {
            let mut channels = self.channels.lock().unwrap();
            if channels.contains_key(name) {
                return Err("Channel already exists");
            }
            let vxchan = Arc::new(Mutex::new(VXChan::new()));
            channels.insert(name.to_string(), vxchan);
            Ok(())
        }

        pub fn send_message(&self, name: &str, message: String) -> Result<(), &'static str> {
            let channels = self.channels.lock().unwrap();
            if let Some(vxchan) = channels.get(name) {
                let vxchan = vxchan.lock().unwrap();
                vxchan.send(message)
            } else {
                Err("Channel not found")
            }
        }

        pub fn receive_message(&self, name: &str) -> Result<String, &'static str> {
            let channels = self.channels.lock().unwrap();
            if let Some(vxchan) = channels.get(name) {
                let vxchan = vxchan.lock().unwrap();
                vxchan.receive()
            } else {
                Err("Channel not found")
            }
        }
    }

    pub fn vxchan_init() -> Result<VXChanManager, &'static str> {
        // Initialize the VXChan module with detailed functionality
        println!("Initializing VXChan module...");
        Ok(VXChanManager::new())
    }
}
