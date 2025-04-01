pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/ 9.0)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
