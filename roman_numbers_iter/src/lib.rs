use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>, pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumberIterator {
    pub value: u32,
    pub roman_digit: RomanDigit,
    pub roman_number: Option<RomanDigit>,
}

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let initial_value = value;
        let mut result = Vec::new();

        let values = [
            RomanNumberIterator {value: 1000, roman_digit: RomanDigit::M, roman_number: None},
            RomanNumberIterator {value: 900, roman_digit: RomanDigit::C, roman_number: Some(RomanDigit::M)},
            RomanNumberIterator {value: 500, roman_digit: RomanDigit::D, roman_number: None},
            RomanNumberIterator {value: 400, roman_digit: RomanDigit::C, roman_number: Some(RomanDigit::D)},
            RomanNumberIterator {value: 100, roman_digit: RomanDigit::C, roman_number: None},
            RomanNumberIterator {value: 90, roman_digit: RomanDigit::X, roman_number: Some(RomanDigit::C)},
            RomanNumberIterator {value: 50, roman_digit: RomanDigit::L, roman_number: None},
            RomanNumberIterator {value: 40, roman_digit: RomanDigit::X, roman_number: Some(RomanDigit::L)},
            RomanNumberIterator {value: 10, roman_digit: RomanDigit::X, roman_number: None},
            RomanNumberIterator {value: 9, roman_digit: RomanDigit::I, roman_number: Some(RomanDigit::X)},
            RomanNumberIterator {value: 5, roman_digit: RomanDigit::V, roman_number: None},
            RomanNumberIterator {value: 4, roman_digit: RomanDigit::I, roman_number: Some(RomanDigit::V)},
            RomanNumberIterator {value: 1, roman_digit: RomanDigit::I, roman_number: None},
        ];

        for num in values.iter() {
            while value >= num.value {
                result.push(num.roman_digit);

                if let Some(roman_number) = num.roman_number {
                    result.push(roman_number);
                }

                value -= num.value;
            }
        }

        RomanNumber(result, initial_value)
    }
}

impl fmt::Debug for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RomanNumber")
            .field(&self.0)
            .finish()
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;

        let new_number = RomanNumber::from(self.1).0;
        self.0 = new_number.clone();

        Some(RomanNumber(new_number, self.1))
    }
}
