// vxde.rs

// Desktop Environment foundation module
pub mod vxde {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    struct VXDE {
        sessions: Arc<Mutex<HashMap<String, Session>>>,
    }

    struct Session {
        id: String,
        user: String,
        status: String,
    }

    impl VXDE {
        pub fn new() -> Self {
            VXDE {
                sessions: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        pub fn create_session(&self, id: String, user: String) {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(id, Session { id, user, status: String::from("active") });
        }

        pub fn terminate_session(&self, id: &str) {
            let mut sessions = self.sessions.lock().unwrap();
            if let Some(session) = sessions.get_mut(id) {
                session.status = String::from("terminated");
            }
        }

        pub fn get_session(&self, id: &str) -> Option<Session> {
            let sessions = self.sessions.lock().unwrap();
            sessions.get(id).cloned()
        }
    }
}
