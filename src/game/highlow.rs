// a simple 1 player higher-or-lower game

use std::cmp::Ordering;

use crate::game::Game;

use rand::prelude::*;

use crate::card::standard::{self, Card, Rank, Suit};
use crate::card::ConditionalOrd;

impl ConditionalOrd for Suit {
    // Leading card, optional trumps suit
    type Info = ();

    fn compare(&self, _: &Self, _: &Self::Info) -> Ordering {
        Ordering::Equal
    }
}

impl Rank {
    fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl ConditionalOrd for Rank {
    // No info needed for Whist
    type Info = ();

    fn compare(&self, other: &Self, _: &Self::Info) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl ConditionalOrd for Card {
    // Leading card, optional trumps suit
    type Info = ();

    fn compare(&self, other: &Self, _: &Self::Info) -> Ordering {
        self.rank.compare(&other.rank, &())
    }
}

pub struct HighLow {
    deck: Vec<Card>,
    card: Card,
    score: u8,
    rng: SmallRng,
}

impl HighLow {
    pub fn new() -> Self {
        let mut deck = standard::deck().to_vec();
        let mut rng = SmallRng::from_entropy();

        deck.shuffle(&mut rng);
        let card = deck.pop().unwrap();

        Self {
            deck,
            card,
            score: 0,
            rng,
        }
    }

    pub fn score(&self) -> u8 {
        self.score
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action {
    Higher,
    Lower,
}

impl From<u32> for Action {
    fn from(action: u32) -> Self {
        match action {
            0 => Action::Higher,
            1 => Action::Lower,
            _ => panic!("Invalid action"),
        }
    }
}

impl From<Action> for u32 {
    fn from(action: Action) -> Self {
        match action {
            Action::Higher => 0,
            Action::Lower => 1,
        }
    }
}

impl Game for HighLow {
    type Action = Action;
    type Player = u8;
    type Reward = u8;
    type State = Card;

    fn current_player(&self) -> &Self::Player {
        &1
    }

    fn legal_actions(&self) -> Vec<Self::Action> {
        vec![Action::Higher, Action::Lower]
    }

    fn observation(&self) -> Self::State {
        self.card
    }

    fn step(&mut self, action: Self::Action) -> (Self::State, Self::Reward, bool) {
        let card = self.deck.pop();

        if card.is_none() {
            return (self.card, self.score, true);
        }

        let card = card.unwrap();
        let higher = card.compare(&self.card, &());

        if action == Action::Higher && higher == Ordering::Greater {
            self.score += 1;
        } else if action == Action::Lower && higher == Ordering::Less {
            self.score += 1;
        } else {
            self.score = 0;
        }

        self.card = card;

        (self.card, self.score, self.deck.is_empty())
    }

    fn reset(&mut self) {
        self.deck = standard::deck().to_vec();
        self.deck.shuffle(&mut self.rng);
        self.card = self.deck.pop().unwrap();
        self.score = 0;
    }

    fn render(&self) {
        println!("Current card: {}", self.card);
        println!("Score: {}", self.score);
    }
}
