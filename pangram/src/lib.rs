use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = HashSet::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            alphabet.insert(c);
        }
    }
    alphabet.len() == 26
}