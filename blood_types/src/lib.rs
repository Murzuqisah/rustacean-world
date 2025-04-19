use std::str::FromStr;
use std::cmp::Ordering;
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen.cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.ends_with('+') {
            (&s[..s.len()-1], "+")
        } else if s.ends_with('-') {
            (&s[..s.len()-1], "-")
        } else {
            return Err(());
        };
        Ok(BloodType {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        };

        // Check Rh factor compatibility
        let rh_ok = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_types = Self::all_blood_types();
        all_types.into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_types = Self::all_blood_types();
        all_types.into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }

    fn all_blood_types() -> Vec<Self> {
        let mut types = Vec::new();
        for antigen in &[Antigen::A, Antigen::AB, Antigen::B, Antigen::O] {
            for rh in &[RhFactor::Positive, RhFactor::Negative] {
                types.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                });
            }
        }
        types
    }
}
