pub fn num_to_ordinal(x: u32) -> String {
    if x >= 10 && x <= 20 {
        return format!("{}th", x);
    } else {
        match x % 10 {
            1 => format!("{}st", x),
            2 => format!("{}nd", x),
            3 => format!("{}rd", x),
            _ => format!("{}th", x),
        }
    }
}