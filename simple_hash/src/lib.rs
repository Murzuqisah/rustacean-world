use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut res = HashMap::new();
    for &word in words {
        *res.entry(word).or_insert(0) += 1;
    }
    res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

    #[test]
    fn test_word_frequency_counter() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);
        let expected_output: HashMap<&str, usize> = [
            ("this", 2),
            ("is", 2),
            ("a", 2),
            ("very", 2),
            ("basic", 3),
            ("sentence", 1),
            ("with", 1),
            ("only", 1),
            ("few", 1),
            ("repetitions.", 1),
            ("once", 1),
            ("again", 1),
            ("but", 1),
            ("it", 1),
            ("should", 1),
            ("be", 1),
            ("enough", 1),
            ("for", 1),
            ("tests", 1),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(frequency_count, expected_output);
    }

    #[test]
    fn test_nb_distinct_words() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);
        assert_eq!(nb_distinct_words(&frequency_count), 19); // Corrected expected output
    }
}