use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut random = rand::thread_rng();
        match random.gen_range(1..=4) {
            1 => return Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            _ => return Suit::Club,
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for suit"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut random = rand::thread_rng();
        match random.random_range(1..=13) {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
            _ => panic!("Invalid value for rank"),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid value for rank"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card == Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}