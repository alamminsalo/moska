use crate::deck::{Card, Deck};
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct Player {
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
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
}

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Table {
    // Card deck to deal cards from
    pub deck: Deck,

    // Players in table
    pub players: Vec<Player>,

    // Turns played
    pub turn: usize,

    // Rounds played
    pub round: usize,
}

impl Table {
    pub fn new(players: u8) -> Self {
        Table {
            players: vec![Player::new(); players as usize],
            deck: Deck::new(),
            turn: 0,
            round: 0,
        }
    }

    pub fn reset(&mut self) {
        self.deck = Deck::new();

        for player in self.players.iter_mut() {
            player.cards = vec![];
        }
    }

    pub fn player_index(&self) -> usize {
        self.turn % self.players.len()
    }

    pub fn current_player(&mut self) -> Option<&Player> {
        self.players.get(self.player_index())
    }

    pub fn current_player_mut(&mut self) -> Option<&mut Player> {
        let player_index = self.player_index();
        self.players.get_mut(player_index)
    }
}
