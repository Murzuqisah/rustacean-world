pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    value: usize,
    max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, _track_value: &std::rc::Rc<usize>) {
        self.value += 1;

        let percentage = (self.value * 100) / self.max;

        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, _track_value: &std::rc::Rc<usize>) {
        let percentage = (self.value * 100) / self.max;
        self.logger.info(&format!("you are using up to {}% of your quota", percentage));
    }
}
