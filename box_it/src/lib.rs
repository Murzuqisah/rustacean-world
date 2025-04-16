pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s
        .split_whitespace()
        .map(|token| {
            if let Some(stripped) = token.strip_suffix('k') {
                // Parse as float to handle values like 5.5k
                let float_val: f32 = stripped.parse().unwrap_or(0.0);
                (float_val * 1000.0).round() as u32
            } else {
                token.parse::<u32>().unwrap_or(0)
            }
        })
        .collect();

    Box::new(nums)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

