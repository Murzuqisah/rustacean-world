pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
    current: T,
}

impl<T: Clone> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg.clone(),
            beg,
            end,
            step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step.clone() > self.beg.clone() && self.current > self.end)
            || (self.step.clone() < self.beg.clone() && self.current < self.end)
        {
            return None;
        }

        let result = self.current.clone();
        self.current = self.current.clone() + self.step.clone();
        Some(result)
    }
}
