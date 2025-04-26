// vxwall.rs

// Security firewall layer module
pub mod vxwall {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXWall {
        rules: Arc<Mutex<HashMap<String, Rule>>>,
    }

    struct Rule {
        id: String,
        action: String,
        protocol: String,
        source: String,
        destination: String,
    }

    impl VXWall {
        pub fn new() -> Self {
            VXWall {
                rules: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn add_rule(&self, id: String, action: String, protocol: String, source: String, destination: String) {
            let mut rules = self.rules.lock().unwrap();
            rules.insert(id, Rule { id, action, protocol, source, destination });
        }

        pub fn remove_rule(&self, id: &str) {
            let mut rules = self.rules.lock().unwrap();
            rules.remove(id);
        }

        pub fn get_rule(&self, id: &str) -> Option<Rule> {
            let rules = self.rules.lock().unwrap();
            rules.get(id).cloned()
        }
    }
}
