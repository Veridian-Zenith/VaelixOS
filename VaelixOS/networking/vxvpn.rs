// vxvpn.rs

// WireGuard integration module
pub mod vxvpn {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXVPN {
        connections: Arc<Mutex<HashMap<String, Connection>>>,
    }

    struct Connection {
        id: String,
        peer: String,
        public_key: String,
        endpoint: String,
        allowed_ips: Vec<String>,
    }

    impl VXVPN {
        pub fn new() -> Self {
            VXVPN {
                connections: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn add_connection(&self, id: String, peer: String, public_key: String, endpoint: String, allowed_ips: Vec<String>) {
            let mut connections = self.connections.lock().unwrap();
            connections.insert(id, Connection { id, peer, public_key, endpoint, allowed_ips });
        }

        pub fn remove_connection(&self, id: &str) {
            let mut connections = self.connections.lock().unwrap();
            connections.remove(id);
        }

        pub fn get_connection(&self, id: &str) -> Option<Connection> {
            let connections = self.connections.lock().unwrap();
            connections.get(id).cloned()
        }
    }
}
