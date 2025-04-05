use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, usize>) -> usize {
    let mut max_value = usize::MIN;

    for &value in h.values() {
        if value > max_value.try_into().unwrap() {
            max_value = value;
        }
    }
    max_value
}