pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() <= 1 {
        return vec![];
    }

    let mut res = vec![1; arr.len()];

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j {
                res[i] *= arr[j];
            }
        }
    }

    res
}
