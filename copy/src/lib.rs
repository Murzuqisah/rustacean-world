pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = (c.abs() as f64).ln()
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<f64> = a.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap() as f64)
        .collect();

    let exp_values: vec<String> = numbers.iter()
        .map(|&n| n.exp().to_string())
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect();

    (b, ln_values)
}
