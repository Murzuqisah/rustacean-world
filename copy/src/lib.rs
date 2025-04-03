pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let ln = if c == 0 {
        f64::NEG_INFINITY // log(0) is negative infinity
    } else {
        (c.abs() as f64).ln()
    };
    (c, exp, ln)
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<f64> = a.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap() as f64) // Convert each part to f64
        .collect();

    let exp_values: Vec<String> = numbers.iter()
        .map(|&n| n.exp().to_string())
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter()
        .map(|&n| {
            if n == 0 {
                f64::NEG_INFINITY // log(0) is negative infinity
            } else {
                (n.abs() as f64).ln()
            }
        })
        .collect();

    (b, ln_values)
}
