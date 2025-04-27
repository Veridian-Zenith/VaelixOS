// src/kernel/vx_tasklet.rs

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Tasklet {
    id: usize,
    priority: usize,
    task: Box<dyn FnOnce() + Send + 'static>,
}

pub struct TaskletScheduler {
    task_queue: Arc<Mutex<VecDeque<Tasklet>>>,
}

impl Clone for TaskletScheduler {
    fn clone(&self) -> Self {
        TaskletScheduler {
            task_queue: Arc::clone(&self.task_queue),
        }
    }
}

impl TaskletScheduler {
    pub fn new() -> Self {
        TaskletScheduler {
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn add_task(&self, task: Box<dyn FnOnce() + Send + 'static>, priority: usize) {
        let mut queue = self.task_queue.lock().unwrap();
        let tasklet = Tasklet {
            id: queue.len(),
            priority,
            task,
        };
        queue.push_back(tasklet);
        queue.make_contiguous().sort_by_key(|t| t.priority);
    }

    pub fn run(&self) {
        loop {
            let mut queue = self.task_queue.lock().unwrap();
            if let Some(tasklet) = queue.pop_front() {
                drop(queue);
                (tasklet.task)();
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

pub fn vx_tasklet_init() -> TaskletScheduler {
    let scheduler = TaskletScheduler::new();
    let scheduler_clone = scheduler.clone();

    thread::spawn(move || {
        scheduler_clone.run();
    });

    scheduler
}
