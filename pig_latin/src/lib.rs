
pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut result = String::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let chars: Vec<char> = word.chars().collect();

        if vowels.contains(&chars[0]) {
            result.push_str(&format!("{}ay ", word));
            continue;
        }

        if chars.len() >= 3 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
            let prefix = &chars[0..3];
            let rest = &chars[3..];
            result.push_str(&format!("{}{}ay ", rest.iter().collect::<String>(), prefix.iter().collect::<String>()));
            continue;
        }

        let mut index = 0;
        while index < chars.len() && !vowels.contains(&chars[index]) {
            index += 1;
        }

        let (consonant_cluster, rest) = chars.split_at(index);
        result.push_str(&format!("{}{}ay ", rest.iter().collect::<String>(), consonant_cluster.iter().collect::<String>()));
    }

    result.trim_end().to_string()
}
