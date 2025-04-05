pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
pub fn title_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }

    result
}
pub fn change_case(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c.to_uppercase().next().unwrap());
        }
    }
    result
}