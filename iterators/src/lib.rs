#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.v;
        self.v = if v % 2 == 0 { v / 2 } else { 3 * v + 1 };
        Some(v)
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}