use crate::RomanDigit::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let mut result = Vec::new();

        if value == 0 {
            result.push(Nulla);
            return RomanNumber(result);
        }

        while value >= 1000 {
            result.push(M);
            value -= 1000;
        }
        if value >= 900 {
            result.push(C);
            result.push(M);
            value -= 900;
        }
        if value >= 500 {
            result.push(D);
            value -= 500;
        }
        if value >= 400 {
            result.push(C);
            result.push(D);
            value -= 400;
        }
        while value >= 100 {
            result.push(C);
            value -= 100;
        }
        if value >= 90 {
            result.push(X);
            result.push(C);
            value -= 90;
        }
        if value >= 50 {
            result.push(L);
            value -= 50;
        }
        if value >= 40 {
            result.push(X);
            result.push(L);
            value -= 40;
        }
        while value >= 10 {
            result.push(X);
            value -= 10;
        }
        if value >= 9 {
            result.push(I);
            result.push(X);
            value -= 9;
        }
        if value >= 5 {
            result.push(V);
            value -= 5;
        }
        if value >= 4 {
            result.push(I);
            result.push(V);
            value -= 4;
        }
        while value >= 1 {
            result.push(I);
            value -= 1;
        }

        RomanNumber(result)
    }
}
