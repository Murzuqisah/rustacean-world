pub fn first_subword(mut s: String) -> String {
    if s.is_empty() { return s};
    let mut first_subword = String::new();
    
    if s.contains('_') {
        first_subword = s.split('_').next().unwrap().to_string();
    } else {
        let mut chars = s.chars();
        first_subword.push(chars.next().unwrap());

        for c in chars {
            if c.is_uppercase() {
                break;
            }
            first_subword.push(c);
        }
    }

    first_subword
}