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
        let kcal: f64 = food.calories[1]
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);


        calories += kcal * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }

    let round_value = |value: f64| (value * 100.0).round() / 100.0;

    
    json::object! {
        "calories" => round_value(calories),
        "carbs" => round_value(carbs),
        "proteins" => round_value(proteins),
        "fats" => round_value(fats),
    }
}
