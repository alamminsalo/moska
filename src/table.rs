use crate::deck::{Card, Deck};
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct Player {
    pub id: u8,
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player { id, cards: vec![] }
    }

    pub fn hand(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn pop_card(&mut self, index: usize) -> Option<Card> {
        if self.cards.len() < index + 1 {
            None
        } else {
            Some(self.cards.remove(index))
        }
    }

    pub fn card_index(&self, card: &Card) -> Option<usize> {
        self.cards.iter().position(|self_card| self_card == card)
    }
}

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Table {
    // Card deck to deal cards from
    pub deck: Deck,

    // Players in table
    pub players: Vec<Player>,

    // Current player
    pub player_index: usize,

    // Turns played
    pub turn: usize,

    // Rounds played
    pub round: usize,
}

#[wasm_bindgen]
impl Table {
    pub(crate) fn new(players: u8) -> Self {
        Table {
            players: (0..players).into_iter().map(Player::new).collect(),
            deck: Deck::new(),
            player_index: 0,
            turn: 0,
            round: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.deck = Deck::new();

        for player in self.players.iter_mut() {
            player.cards = vec![];
        }
    }

    pub(crate) fn current_player(&mut self) -> Option<&Player> {
        self.players.get(self.player_index)
    }

    pub(crate) fn current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.player_index)
    }

    pub(crate) fn next_turn(&mut self, next_player_index: usize) {
        self.player_index = next_player_index;
        self.turn += 1;
    }
}
