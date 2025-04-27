pub mod vxchan {
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{self, Sender, Receiver};
    use std::thread;

    pub struct VXChan {
        sender: Sender<String>,
        receiver: Receiver<String>,
    }

    impl VXChan {
        pub fn new() -> VXChan {
            let (sender, receiver) = mpsc::channel();
            VXChan { sender, receiver }
        }

        pub fn send(&self, message: String) {
            self.sender.send(message).unwrap();
        }

        pub fn receive(&self) -> String {
            self.receiver.recv().unwrap()
        }
    }

    pub fn vxchan_init() -> Result<(), &'static str> {
        // Initialize the VXChan module with detailed functionality
        println!("Initializing VXChan module...");

        // Example of creating a VXChan instance
        let vxchan = Arc::new(Mutex::new(VXChan::new()));

        // Clone the Arc to share with the spawned thread
        let vxchan_clone = Arc::clone(&vxchan);

        // Spawn a thread to send a message
        thread::spawn(move || {
            let vxchan = vxchan_clone.lock().unwrap();
            vxchan.send("Hello from VXChan!".to_string());
        });

        // Receive the message in the main thread
        let vxchan = vxchan.lock().unwrap();
        let received_message = vxchan.receive();
        println!("Received message: {}", received_message);

        Ok(())
    }
}
