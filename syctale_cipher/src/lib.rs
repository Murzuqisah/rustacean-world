pub fn scytale_cipher(message: String, i: u32) -> String {
   let size = i as usize;
    let mut chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + size - 1) / size;
    let total_len = rows * size;

    while chars.len() < total_len {
        chars.push(' ');
    }

    let mut result = String::new();

    for i in 0..size {
        for j in 0..rows {
            result.push(chars[j * size + i]);
        }
    }

    result.trim_end().to_string()
}

