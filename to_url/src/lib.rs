pub fn to_url(s: &str) -> String {
    s
        .split_whitespace()
        .map(|word| format!("{}", word))
        .collect::<Vec<String>>()
        .join("%20")
}