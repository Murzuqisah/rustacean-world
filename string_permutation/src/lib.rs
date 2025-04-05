use std::collections::HashMap;

pub fn is_permutation(word: &str, word1: &str) -> bool {
    if word.len() != word1.len() {
        return false;
    }

    let mut char_count = HashMap::new();

    for c in word.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    for c in word1.chars() {
        match char_count.get_mut(&c) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    char_count.remove(&c);
                }
            }
            None => return false,
        }
    }

    char_count.is_empty()
}