#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart { items: Vec::new(), receipt: Vec::new() }
    }

    pub fn insert_item(&mut self, store: &Store, name: String) {
        if let Some(item) = store.products.iter().find(|(n, _)| n == &name) {
            self.items.push(item.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut result: Vec<f32> = Vec::new();
        let mut items_arr = self.items.clone();

        items_arr.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let sorted_items = items_arr.clone();

        let free_items = items_arr.len() / 3;
        for ind in 0..free_items {
            items_arr[ind].1 = 0.0;
        }

        let new_coeff = calculate_coeff(&items_arr, &self.items);

        for item in sorted_items {
            let new_price = item.1 * new_coeff;
            result.push(round2d(new_price));
        }

        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = result.clone();
        result
    }
}

fn round2d(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}

fn calculate_coeff(new_items: &Vec<(String, f32)>, old_items: &Vec<(String, f32)>) -> f32 {
    let new_sum: f32 = new_items.iter().map(|(_, price)| price).sum();
    let old_sum: f32 = old_items.iter().map(|(_, price)| price).sum();

    if old_sum == 0.0 {
        1.0
    } else {
        1.0 - (old_sum - new_sum) / old_sum
    }
}
