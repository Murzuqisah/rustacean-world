pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();

    for ch in s.chars() {
        match ch {
            '-' => {
                result.pop();
            }
            '+' => {
            }
            _ => {
                result.push(ch);
            }
        }
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
