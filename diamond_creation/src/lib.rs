pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as usize - 'A' as usize;
    let mut lines = Vec::new();

    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let padding = " ".repeat(n - i);
        let line = if i == 0 {
            format!("{}A{}", padding, padding)
        } else {
            let middle_space = " ".repeat(i * 2 - 1);
            format!("{}{}{}{}{}", padding, letter, middle_space, letter, padding)
        };
        lines.push(line);
    }

    for i in (0..n).rev() {
        lines.push(lines[i].clone());
    }

    lines
}
