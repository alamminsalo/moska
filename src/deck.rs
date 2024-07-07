use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
#[wasm_bindgen]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[wasm_bindgen]
pub enum Rank {
    Joker = 0,
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct Card {
    #[wasm_bindgen(skip)]
    pub suit: Suit,

    #[wasm_bindgen(skip)]
    pub rank: Rank,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    #[wasm_bindgen(getter)]
    pub fn suit(&self) -> String {
        format!("{:?}", self.suit)
    }

    #[wasm_bindgen(getter)]
    pub fn rank(&self) -> String {
        match self.rank {
            Rank::Joker => "D".to_string(),
            Rank::Ace => "A".to_string(),
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
            _ => format!("{}", self.rank as u8),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn unicode(&self) -> String {
        use Rank::*;
        use Suit::*;

        match (self.rank, self.suit) {
            (Ace, Spades) => "\u{1F0A1}",
            (Two, Spades) => "\u{1F0A2}",
            (Three, Spades) => "\u{1F0A3}",
            (Four, Spades) => "\u{1F0A4}",
            (Five, Spades) => "\u{1F0A5}",
            (Six, Spades) => "\u{1F0A6}",
            (Seven, Spades) => "\u{1F0A7}",
            (Eight, Spades) => "\u{1F0A8}",
            (Nine, Spades) => "\u{1F0A9}",
            (Ten, Spades) => "\u{1F0AA}",
            (Jack, Spades) => "\u{1F0AB}",
            (Queen, Spades) => "\u{1F0AD}",
            (King, Spades) => "\u{1F0AE}",

            (Ace, Hearts) => "\u{1F0B1}",
            (Two, Hearts) => "\u{1F0B2}",
            (Three, Hearts) => "\u{1F0B3}",
            (Four, Hearts) => "\u{1F0B4}",
            (Five, Hearts) => "\u{1F0B5}",
            (Six, Hearts) => "\u{1F0B6}",
            (Seven, Hearts) => "\u{1F0B7}",
            (Eight, Hearts) => "\u{1F0B8}",
            (Nine, Hearts) => "\u{1F0B9}",
            (Ten, Hearts) => "\u{1F0BA}",
            (Jack, Hearts) => "\u{1F0BB}",
            (Queen, Hearts) => "\u{1F0BD}",
            (King, Hearts) => "\u{1F0BE}",

            (Ace, Diamonds) => "\u{1F0C1}",
            (Two, Diamonds) => "\u{1F0C2}",
            (Three, Diamonds) => "\u{1F0C3}",
            (Four, Diamonds) => "\u{1F0C4}",
            (Five, Diamonds) => "\u{1F0C5}",
            (Six, Diamonds) => "\u{1F0C6}",
            (Seven, Diamonds) => "\u{1F0C7}",
            (Eight, Diamonds) => "\u{1F0C8}",
            (Nine, Diamonds) => "\u{1F0C9}",
            (Ten, Diamonds) => "\u{1F0CA}",
            (Jack, Diamonds) => "\u{1F0CB}",
            (Queen, Diamonds) => "\u{1F0CD}",
            (King, Diamonds) => "\u{1F0CE}",

            (Ace, Clubs) => "\u{1F0D1}",
            (Two, Clubs) => "\u{1F0D2}",
            (Three, Clubs) => "\u{1F0D3}",
            (Four, Clubs) => "\u{1F0D4}",
            (Five, Clubs) => "\u{1F0D5}",
            (Six, Clubs) => "\u{1F0D6}",
            (Seven, Clubs) => "\u{1F0D7}",
            (Eight, Clubs) => "\u{1F0D8}",
            (Nine, Clubs) => "\u{1F0D9}",
            (Ten, Clubs) => "\u{1F0DA}",
            (Jack, Clubs) => "\u{1F0DB}",
            (Queen, Clubs) => "\u{1F0DD}",
            (King, Clubs) => "\u{1F0DE}",

            (Joker, _) => "\u{1F0DF}", // white joker symbol
        }
        .to_string()
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Deck {
    deck: Vec<Card>,
}

#[wasm_bindgen]
impl Deck {
    pub fn new() -> Self {
        let mut deck = vec![];
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 1..=13 {
                deck.push(Card::new(suit, Rank::try_from(rank).unwrap()));
            }
        }
        Self { deck }
    }

    pub fn new_with_jokers() -> Self {
        let mut deck = vec![];
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 0..=13 {
                deck.push(Card::new(suit, Rank::try_from(rank).unwrap()));
            }
        }
        Self { deck }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub(crate) fn pop(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    pub(crate) fn pop_last(&mut self) -> Option<Card> {
        if self.deck.is_empty() {
            None
        } else {
            Some(self.deck.remove(0))
        }
    }

    // Peeks first card from the top of the deck
    pub(crate) fn peek(&mut self) -> Option<&Card> {
        self.deck.last()
    }

    // Peeks last card from the bottom of the deck
    pub(crate) fn peek_last(&mut self) -> Option<&Card> {
        self.deck.first()
    }

    pub fn push(&mut self, card: Card) {
        self.deck.push(card);
    }

    #[wasm_bindgen(getter)]
    pub fn count(&self) -> usize {
        self.deck.len()
    }
}

impl TryFrom<u8> for Rank {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::Joker),
            1 => Ok(Rank::Ace),
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            _ => Err("Invalid value for Rank"),
        }
    }
}

use std::fmt::{self, Display};

fn suit_symbol(suit: Suit) -> String {
    match suit {
        Suit::Spades => "\u{2660}",
        Suit::Hearts => "\u{2665}",
        Suit::Diamonds => "\u{2666}",
        Suit::Clubs => "\u{2663}",
    }
    .to_string()
}

pub fn suit_string(suit: Suit) -> String {
    format!("{:?}", suit)
}

pub fn rank_fmt(rank: Rank) -> String {
    match rank {
        Rank::Joker => "D".to_string(),
        Rank::Ace => "A".to_string(),
        Rank::King => "K".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::Jack => "J".to_string(),
        _ => format!("{}", rank as u8),
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", suit_symbol(*self))
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", rank_fmt(*self))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
