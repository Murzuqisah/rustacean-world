extern crate json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut calories = 0.0;
    let mut fats = 0.0; 
    let mut carbs = 0.0;
    let mut proteins = 0.0;

    for food in foods {
        let kcal_str = &food.calories[1];
        let kcal: f64 = kcal_str.trim_end_matches("kcal").parse::<f64>().unwrap();

        calories += kcal * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }

    fn round_value(val: f64) -> f64 {
        let rounded = (val * 100.0).round() / 100.0;
        if (rounded * 10.0).round() / 10.0 == rounded {
            (val * 10.0).round() / 10.0
        } else {
            rounded
        }
    }

    json::object! {
        "calories" => round_value(calories),
        "fats" => round_value(fats),
        "carbs" => round_value(carbs),
        "proteins" => round_value(proteins),
    }
}