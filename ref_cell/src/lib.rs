use std::cell::RefCell;
use std::collections::HashMap;

pub mod messenger;

pub use messenger::{Logger, Tracker};

pub struct Worker {
    pub track_value: std::rc::Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(initial_value: usize) -> Self {
        Worker {
            track_value: std::rc::Rc::new(initial_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(format!("Error: {}", msg));
    }
}
