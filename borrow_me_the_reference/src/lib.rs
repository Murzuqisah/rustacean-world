pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '-' => {
                result.pop();
            }
            '+' => {
                let mut count = 1;
                let mut j = i + 1;
                while j < chars.len() && chars[j] == '+' {
                    count += 2;
                    j += 1;
                }
                i = i + count;
            }
            c => {
                result.push(c);
            }
        }
        i += 1;
    }

    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for eq in v.iter_mut() {
        let mut result = 0;
        let mut current_number = String::new();
        let mut operator = '+';

        for ch in eq.chars() {
            if ch == '+' || ch == '-' {
                if !current_number.is_empty() {
                    let num: i32 = current_number.parse().unwrap();
                    result = if operator == '+' { result + num } else { result - num };
                }
                operator = ch;
                current_number.clear();
            } else {
                current_number.push(ch);
            }
        }
        
        if !current_number.is_empty() {
            let num: i32 = current_number.parse().unwrap();
            result = if operator == '+' { result + num } else { result - num };
        }

        *eq = result.to_string();
    }
}
