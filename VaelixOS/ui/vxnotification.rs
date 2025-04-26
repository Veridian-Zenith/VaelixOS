// vxnotification.rs

// Notification system module
pub mod vxnotification {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXNotification {
        notifications: Arc<Mutex<HashMap<String, Notification>>>,
    }

    struct Notification {
        id: String,
        message: String,
        timestamp: u64,
    }

    impl VXNotification {
        pub fn new() -> Self {
            VXNotification {
                notifications: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn create_notification(&self, id: String, message: String, timestamp: u64) {
            let mut notifications = self.notifications.lock().unwrap();
            notifications.insert(id, Notification { id, message, timestamp });
        }

        pub fn remove_notification(&self, id: &str) {
            let mut notifications = self.notifications.lock().unwrap();
            notifications.remove(id);
        }

        pub fn get_notification(&self, id: &str) -> Option<Notification> {
            let notifications = self.notifications.lock().unwrap();
            notifications.get(id).cloned()
        }
    }
}
