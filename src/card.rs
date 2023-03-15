use std::cmp::Ordering;

pub trait ConditionalOrd {
    type Info;

    fn compare(&self, other: &Self, info: &Self::Info) -> Ordering;
}

#[derive(Debug, Clone, Copy)]
pub struct BaseCard<S, R>
where
    S: ConditionalOrd + Sized,
    R: ConditionalOrd + Sized,
{
    pub suit: S,
    pub rank: R,
}

impl<S, R> BaseCard<S, R>
where
    S: ConditionalOrd + Sized,
    R: ConditionalOrd + Sized,
{
    pub fn new(suit: S, rank: R) -> BaseCard<S, R> {
        BaseCard { suit, rank }
    }
}

pub mod standard {
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Suit {
        Hearts,
        Clubs,
        Diamonds,
        Spades,
    }

    impl Display for Suit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Suit::Hearts => write!(f, "Hearts"),
                Suit::Clubs => write!(f, "Clubs"),
                Suit::Diamonds => write!(f, "Diamonds"),
                Suit::Spades => write!(f, "Spades"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Rank {
        Ace,
        King,
        Queen,
        Jack,
        Ten,
        Nine,
        Eight,
        Seven,
        Six,
        Five,
        Four,
        Three,
        Two,
    }

    impl Display for Rank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Rank::Ace => write!(f, "Ace"),
                Rank::Two => write!(f, "Two"),
                Rank::Three => write!(f, "Three"),
                Rank::Four => write!(f, "Four"),
                Rank::Five => write!(f, "Five"),
                Rank::Six => write!(f, "Six"),
                Rank::Seven => write!(f, "Seven"),
                Rank::Eight => write!(f, "Eight"),
                Rank::Nine => write!(f, "Nine"),
                Rank::Ten => write!(f, "Ten"),
                Rank::Jack => write!(f, "Jack"),
                Rank::Queen => write!(f, "Queen"),
                Rank::King => write!(f, "King"),
            }
        }
    }

    pub type Card = super::BaseCard<Suit, Rank>;

    impl From<Card> for u32 {
        fn from(card: Card) -> Self {
            (card.suit as u32) * 13 + (card.rank as u32)
        }
    }

    impl PartialEq for Card {
        fn eq(&self, other: &Self) -> bool {
            self.rank == other.rank && self.suit == other.suit
        }
    }

    impl Display for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} of {}", self.rank, self.suit)
        }
    }

    pub fn deck() -> [Card; 52] {
        [Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades]
            .iter()
            .flat_map(|suit| {
                [
                    Rank::Ace,
                    Rank::Two,
                    Rank::Three,
                    Rank::Four,
                    Rank::Five,
                    Rank::Six,
                    Rank::Seven,
                    Rank::Eight,
                    Rank::Nine,
                    Rank::Ten,
                    Rank::Jack,
                    Rank::Queen,
                    Rank::King,
                ]
                .iter()
                .map(move |rank| Card::new(*suit, *rank))
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}
