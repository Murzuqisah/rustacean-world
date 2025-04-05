use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len() as f64;
    sum as f64 / count
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort();
    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        (sorted_list[mid - 1] + sorted_list[mid]) / 2
    } else {
        sorted_list[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {

    let mut occurrences = HashMap::new();
    for &value in list {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap_or(0)
}