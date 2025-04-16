pub mod messenger;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use messenger::Logger;

pub struct Worker {
    pub track_value: Rc<u8>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: u8) -> Self {
        Self {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Warning: {}", msg));
    }

    fn info(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Info: {}", msg));
    }

    fn error(&self, msg: &str) {
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());
        self.all_messages
            .borrow_mut()
            .push(format!("Error: {}", msg));
    }
}
