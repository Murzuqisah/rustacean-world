
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // Create a mutable list of field-value pairs
        let mut components = [
            ("r", self.r),
            ("g", self.g),
            ("b", self.b),
            ("a", self.a),
        ];

        // Find indices of fields matching the given values
        let mut first_index = None;
        let mut second_index = None;

        for (i, &(_, val)) in components.iter().enumerate() {
            if val == first && first_index.is_none() {
                first_index = Some(i);
            } else if val == second && second_index.is_none() {
                second_index = Some(i);
            }
        }

        // If both found, swap
        if let (Some(i), Some(j)) = (first_index, second_index) {
            components.swap(i, j);
        }

        // Assign the new values back
        self.r = components[0].1;
        self.g = components[1].1;
        self.b = components[2].1;
        self.a = components[3].1;

        self
    }
}
