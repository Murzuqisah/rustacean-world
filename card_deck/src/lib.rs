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
        let random = Self::generate_random_number(1, 4);
        Self::translate(random)
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

    fn generate_random_number(min: u8, max: u8) -> u8 {
        let now = std::time::SystemTime::now();
        let since_the_epoch = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u8;
        (since_the_epoch % (max - min + 1)) + min
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random = Self::generate_random_number(1, 13);
        Self::translate(random)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n if n >= 2 && n <= 10 => Rank::Number(n),
            _ => panic!("Invalid value for rank"),
        }
    }

    fn generate_random_number(min: u8, max: u8) -> u8 {
        let now = std::time::SystemTime::now();
        let since_the_epoch = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as u8;
        (since_the_epoch % (max - min + 1)) + min
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    matches!(card.rank, Rank::Ace) && matches!(card.suit, Suit::Spade)
}