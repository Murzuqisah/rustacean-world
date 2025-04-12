
pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let a = 'a' as u8;
                let rotated = ((c as u8 - a + (key.rem_euclid(26) as u8)) % 26) + a;
                rotated as char
            } else if c.is_ascii_uppercase() {
                let a = 'A' as u8;
                let rotated = ((c as u8 - a + (key.rem_euclid(26) as u8)) % 26) + a;
                rotated as char
            } else {
                c
            }
        })
        .collect()
}
