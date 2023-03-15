use rand::prelude::*;
use std::cmp::Ordering;

use crate::card::standard;
use crate::card::standard::{Card, Rank, Suit};
use crate::card::ConditionalOrd;
use crate::game::Game;

impl ConditionalOrd for Suit {
    // Leading card, optional trumps suit
    type Info = (Suit, Option<Suit>);

    fn compare(&self, other: &Self, info: &Self::Info) -> Ordering {
        let (leading, trumps) = info;
        let trumps = trumps.unwrap_or(*leading);

        if self == other {
            return Ordering::Equal;
        }

        if other == &trumps {
            return Ordering::Less;
        }

        Ordering::Greater
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
    type Info = (Suit, Option<Suit>);

    fn compare(&self, other: &Self, info: &Self::Info) -> Ordering {
        match self.suit.compare(&other.suit, info) {
            Ordering::Equal => self.rank.compare(&other.rank, &()),
            other => other,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    id: u32,
    hand: [Option<Card>; 13],
    score: u8,
}

impl Player {
    pub fn new(id: u32) -> Player {
        Player {
            id: id,
            hand: [None; 13],
            score: 0,
        }
    }
}

impl From<Player> for u32 {
    fn from(player: Player) -> u32 {
        player.id as u32
    }
}

pub struct Whist {
    players: [Player; 4],
    trick: Vec<Card>,
    seen: Vec<Card>,
    trumps: Option<Suit>,
    deck: [Card; 52],
    rng: ThreadRng,
}

impl Whist {
    pub fn new() -> Whist {
        let mut rng = rand::thread_rng();
        let deck = standard::deck();
        let mut players = [
            Player::new(0),
            Player::new(1),
            Player::new(2),
            Player::new(3),
        ];

        let mut suits = vec![
            Some(Suit::Hearts),
            Some(Suit::Clubs),
            Some(Suit::Diamonds),
            Some(Suit::Spades),
            None,
        ];
        suits.shuffle(&mut rng);

        Whist {
            players,
            trick: Vec::new(),
            seen: Vec::new(),
            trumps: suits[0],
            deck,
            rng,
        }
    }

    fn deal(&mut self) {
        self.deck.shuffle(&mut self.rng);

        for (i, card) in self.deck.iter().enumerate() {
            self.players[i % 4].hand[i / 4] = Some(*card);
        }
    }
}

impl Game for Whist {
    type Action = u8;
    type Player = Player;
    type Reward = u8;
    type State = ([Option<Card>; 13], Vec<Card>, Option<Suit>, Vec<Card>);

    fn current_player(&self) -> &Player {
        self.players.first().unwrap()
    }

    fn observation(&self) -> Self::State {
        // observation = (hand, seen, trumps, trick)

        (
            self.current_player().hand,
            self.seen.clone(),
            self.trumps,
            self.trick.clone(),
        )
    }

    fn legal_actions(&self) -> Vec<Self::Action> {
        let player = self.current_player();

        let mut actions: Vec<_> = player
            .hand
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_some())
            .map(|(i, _)| i as Self::Action)
            .collect();

        if self.trick.is_empty() {
            return actions;
        }

        let leading_suit = self.trick.first().expect("trick should not be empty").suit;

        let has_leading = player.hand.iter().any(|c| match c {
            Some(c) => c.suit == leading_suit,
            None => false,
        });

        if !has_leading {
            return actions;
        }

        actions.retain(|i| player.hand[*i as usize].unwrap().suit == leading_suit);

        actions
    }

    fn step(&mut self, action: Self::Action) -> (Self::State, Self::Reward, bool) {
        // returns (observation, reward, done)
        let mut player = self.players.first_mut().unwrap();
        let card = player.hand[action as usize].unwrap();

        self.trick.push(card);
        self.seen.push(card);

        player.hand[action as usize] = None;

        if self.trick.len() != 4 {
            self.players.rotate_left(1);
            return (self.observation(), 0, false);
        }

        let leading = self.trick.first().unwrap();
        let mut trick = self.trick.clone();
        trick.sort_by(|a, b| a.compare(b, &(leading.suit, self.trumps)));

        let winner = self
            .trick
            .iter()
            .position(|c| c == trick.last().unwrap())
            .unwrap();

        self.players.rotate_left(winner + 1);
        self.players[0].score += 1;

        self.trick.clear();

        (self.observation(), 0, false)
    }

    fn reset(&mut self) {
        *self = Whist::new();
    }

    fn render(&self) {
        let player = self.current_player();

        println!("Player: {:?}", player);
        println!("Trick: {:?}", self.trick);
        println!("Trump: {:?}", self.trumps);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut whist = Whist::new();
        whist.deal();

        let mut seen: Vec<Card> = Vec::new();

        for player in whist.players.iter() {
            for card in player.hand.iter() {
                assert!(card.is_some());
                assert!(!seen.contains(&card.unwrap()));
                seen.push(card.unwrap());
            }
        }
    }

    #[test]
    fn test_legal_actions() {
        let mut whist = Whist::new();
        whist.deal();

        let player = whist.current_player();

        for card in player.hand.iter() {
            if let Some(card) = card {
                println!("{}", card.suit);
            } else {
                println!("None");
            }
        }

        let actions = whist.legal_actions();
        assert_eq!(actions.len(), 13);

        println!("actions: {:?}", actions);

        whist.trick.push(Card {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        });

        let actions = whist.legal_actions();
        println!("actions: {:?}", actions);
    }

    #[test]
    fn test_step() {
        let mut whist = Whist::new();
        println!("TRUMPS ARE: {:?}", whist.trumps);
        whist.deal();

        for _ in 0..13 {
            for _ in 0..4 {
                whist.render();
                let actions = whist.legal_actions();
                whist.step(actions[0]);
            }

            println!("");
        }

        assert!(false);
    }
}
