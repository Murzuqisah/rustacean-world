pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for c in 0..array.len() {
        if array[c] == key {
            return Some(c);
        }
    }
    None
}