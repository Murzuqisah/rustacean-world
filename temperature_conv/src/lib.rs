pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32 as f64) * 0.5555555555555555
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9 as f64 / 5 as f64)) + 32 as f64
}