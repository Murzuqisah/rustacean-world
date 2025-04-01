pub fn sum(a: i16, b: i16) -> i16 {
    a + b
}

pub fn diff(a: i32, b: i32) -> i32 {
    a - b
}

pub fn pro(a: i32, b: i32) -> i32 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f64, b: f64) -> f64 {
    a % b
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}