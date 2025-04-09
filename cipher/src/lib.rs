#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    fn atbash_encode(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    if c.is_lowercase() {
                        ('z' as u8 - c as u8 + 'a' as u8) as char
                    } else {
                        ('Z' as u8 - c as u8 + 'A' as u8) as char
                    }
                } else {
                    c
                }
            })
            .collect()
    }

    let expected_cipher = atbash_encode(original);

    if expected_cipher == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: expected_cipher,
        })
    }
}