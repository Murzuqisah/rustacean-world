use std::f64::consts::E;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;
use std::f64::consts::LN_2;
use std::f64::consts::LN_10;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = if c == 0{
        NEG_INFINITY
    } else {
        c.abs() as f64
    };

    return (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a
        .split_whitespace()
        .map(|s| {
            let number: f64 = s.parse().unwrap();
            format!("{}", number * number)
        })
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect();

    (b, ln_values)
}
