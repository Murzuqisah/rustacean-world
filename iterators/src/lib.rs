#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            return None;
        } else if self.v % 2 == 0 {
            self.v /= 2;
            Some(Collatz::new(self.v * 2))
        } else {
            self.v = self.v * 3 + 1;
            Some(Collatz::new((self.v - 1) / 3))
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }

    let mut count = 0;
    let mut collatz_strt = Collatz::new(n);
    while collatz_strt.v != 1 {
        count += 1;
        if collatz_strt.v % 2 == 0 {
            collatz_strt.v = collatz_strt.v / 2;
        } else {
            collatz_strt.v = collatz_strt.v * 3 + 1;
        }
    }

    count
}
