pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = [false; 26];
    for c in s.to_lowercase().chars() {
        if c.is_alphabetic() {
            let index = (c as u8 - b'a') as usize;
            alphabet[index] = true;
        }
    }
    alphabet.iter().all(|&x| x)
}