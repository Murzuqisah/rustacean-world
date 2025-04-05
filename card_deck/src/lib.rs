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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_translate() {
        assert!(matches!(Suit::translate(1), Suit::Heart));
        assert!(matches!(Suit::translate(2), Suit::Diamond));
        assert!(matches!(Suit::translate(3), Suit::Spade));
        assert!(matches!(Suit::translate(4), Suit::Club));
        assert!(std::panic::catch_unwind(|| Suit::translate(0)).is_err());
        assert!(std::panic::catch_unwind(|| Suit::translate(5)).is_err());
    }

    #[test]
    fn test_rank_translate() {
        assert!(matches!(Rank::translate(1), Rank::Ace));
        assert!(matches!(Rank::translate(11), Rank::Jack));
        assert!(matches!(Rank::translate(12), Rank::Queen));
        assert!(matches!(Rank::translate(13), Rank::King));
        assert!(matches!(Rank::translate(2), Rank::Number(2)));
        assert!(matches!(Rank::translate(10), Rank::Number(10)));
        assert!(std::panic::catch_unwind(|| Rank::translate(0)).is_err());
        assert!(std::panic::catch_unwind(|| Rank::translate(14)).is_err());
    }

    #[test]
    fn test_winner_card() {
        let ace_of_spades = Card { suit: Suit::Spade, rank: Rank::Ace };
        let king_of_hearts = Card { suit: Suit::Heart, rank: Rank::King };
        assert!(winner_card(ace_of_spades));
        assert!(!winner_card(king_of_hearts));
    }

    #[test]
    fn test_random_suit_and_rank() {
        for _ in 0..100 {
            let suit = Suit::random();
            assert!(matches!(suit, Suit::Heart) || matches!(suit, Suit::Diamond) || matches!(suit, Suit::Spade) || matches!(suit, Suit::Club));

            let rank = Rank::random();
            assert!(matches!(rank, Rank::Ace) || matches!(rank, Rank::King) || matches!(rank, Rank::Queen) || matches!(rank, Rank::Jack) || matches!(rank, Rank::Number(_)));
        }
    }
}