/*
 * Game logic for Finnish Moska
 */

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::{
    deck::{Card, Rank, Suit},
    table::Table,
};

#[derive(Debug)]
enum PlayerAction {
    AddCard = 1,
    TakeCard = 2,
    Submit = 3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[wasm_bindgen]
pub enum State {
    Initial,
    PlayerAttacking,
    PlayerDefending,
    GameOver,
}

#[wasm_bindgen(getter_with_clone)]
pub struct Moska {
    pub table: Table,
    pub trump_card: Card,

    pub attacker_cards: Vec<Card>,
    pub defender_cards: Vec<Card>,
    pub discarded: Vec<Card>,

    pub state: State,
}

#[wasm_bindgen]
impl Moska {
    #[wasm_bindgen(constructor)]
    pub fn new(players: u8) -> Self {
        Self {
            table: Table::new(players),
            trump_card: Card::new(Suit::Hearts, Rank::Two),
            attacker_cards: vec![],
            defender_cards: vec![],
            discarded: vec![],
            state: State::Initial,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.table.reset();
        self.attacker_cards.clear();
        self.defender_cards.clear();
        self.discarded.clear();
        self.table.deck.shuffle();
        self.state = State::Initial;
    }

    // Finds next player with cards left
    pub fn next_player(&self) -> usize {
        let num_players = self.table.players.len();
        let mut player_index = (self.table.player_index + 1) % num_players;

        while self.table.players[player_index].cards.len() == 0 {
            // check looped all players
            if player_index == self.table.player_index {
                break;
            }

            player_index = (player_index + 1) % num_players;
        }

        player_index
    }

    pub fn new_round(&mut self) {
        self.table.round += 1;

        self.setup();

        self.state = State::PlayerAttacking;
    }

    // Continues to next turn.
    // Sets state to defending if there are attacking cards present.
    fn next_turn(&mut self) -> bool {
        // Check game ending state:
        // A single player left with cards in their hand
        let players_with_cards: Vec<usize> = self
            .table
            .players
            .iter()
            .enumerate()
            .filter_map(|(index, p)| p.cards.first().and_then(|_| Some(index)))
            .collect();

        if players_with_cards.len() == 1 {
            self.state = State::GameOver;
            return true;
        }

        // Set state depending if attacking cards are present
        if self.attacker_cards.is_empty() {
            self.state = State::PlayerAttacking;
        } else {
            self.state = State::PlayerDefending;
        }

        // Proceed to next turn
        self.table.next_turn(self.next_player());

        true
    }

    pub fn player_action(&mut self, action: usize, card_index: usize) -> bool {
        // Puts the selected card to the table
        if action == PlayerAction::AddCard as usize {
            if let Some(player) = self.table.current_player_mut() {
                if let Some(card) = player.pop_card(card_index) {
                    match &self.state {
                        State::PlayerAttacking => {
                            self.attacker_cards.push(card);
                        }
                        State::PlayerDefending => {
                            self.defender_cards.push(card);
                        }
                        _ => {
                            // NOOP
                        }
                    }
                }
            }
        }

        // Takes the last placed card from table
        // from attacking or defending cards,
        // depending on the current state.
        if action == PlayerAction::TakeCard as usize {
            if let Some(player) = self.table.current_player_mut() {
                match &self.state {
                    State::PlayerAttacking => {
                        if self.attacker_cards.get(card_index).is_some() {
                            player.cards.push(self.attacker_cards.remove(card_index));
                        }
                    }
                    State::PlayerDefending => {
                        if self.defender_cards.get(card_index).is_some() {
                            player.cards.push(self.defender_cards.remove(card_index));
                        }
                    }
                    _ => {
                        // NOOP
                    }
                }
            }
        }

        // Submit cards on the table.
        //
        // Defending:
        // Either ends turn prematurely if no defending cards present
        // or attemps to resolve the table.
        // Continues to attacking state if defending succeeds.
        //
        // Attacking:
        // Checks attacking cards validity and starts next turn,
        // with next player defending.
        if action == PlayerAction::Submit as usize {
            match self.state {
                State::PlayerDefending => {
                    if self.defender_cards.is_empty() {
                        // Take all the attacking cards and continue to next turn.
                        self.table
                            .current_player_mut()
                            .unwrap()
                            .cards
                            .extend(self.attacker_cards.drain(..));

                        self.draw_cards();
                        return self.next_turn();
                    }

                    self.eval_defense().then(|| {
                        self.discard_table();
                        self.draw_cards();
                        self.state = State::PlayerAttacking;

                        // check if player holds any cards
                        if self.table.current_player().unwrap().cards.is_empty() {
                            self.next_turn();
                        }
                    });
                }
                State::PlayerAttacking => {
                    self.eval_attack().then(|| {
                        self.draw_cards();
                        self.next_turn();
                    });
                }
                _ => {}
            }
        }

        true
    }

    // Returns copy of player cards
    pub fn player_cards(&self, player_index: usize) -> Vec<Card> {
        self.table
            .players
            .get(player_index)
            .and_then(|player| Some(player.cards.clone()))
            .unwrap_or_default()
    }

    // Sets up a new game
    fn setup(&mut self) {
        self.reset();

        // Deal 6 cards for each player
        for player in &mut self.table.players {
            for _ in 0..6 {
                player.cards.push(
                    self.table
                        .deck
                        .pop()
                        .expect("Failed to get card while dealing"),
                );
            }
        }

        // Draw the trump card
        self.trump_card = self.table.deck.peek_last().cloned().unwrap();
    }

    // Draws enough cards for player until deck is empty
    fn draw_cards(&mut self) {
        let player_index = self.table.player_index;
        if let Some(_) = self.table.current_player() {
            let player = self.table.players.get_mut(player_index).unwrap();
            while player.cards.len() < 6 && self.table.deck.peek().is_some() {
                if let Some(card) = self.table.deck.pop() {
                    player.cards.push(card);
                }
            }
        }
    }

    #[wasm_bindgen(getter)]
    pub fn valid(&self) -> bool {
        use State::*;
        match self.state {
            PlayerAttacking => self.eval_attack(),
            PlayerDefending => self.eval_defense(),
            _ => false,
        }
    }

    // Evaluates attacking validity.
    // Attacking cards must either contain a single card
    // or must be paired cards.
    fn eval_attack(&self) -> bool {
        if self.attacker_cards.is_empty() {
            return false;
        }

        // Single card
        if self.attacker_cards.len() == 1 {
            return true;
        }

        // Multiple cards:
        // Count card ranks using map
        // -> should have no orphan cards
        let mut map = HashMap::new();
        for card in self.attacker_cards.iter() {
            let key = card.rank as u8;

            if !map.contains_key(&key) {
                map.insert(card.rank as u8, 1);
            } else {
                *map.get_mut(&key).unwrap() += 1;
            }
        }

        if map.values().any(|val| *val < 2) {
            return false;
        }

        true
    }

    // Resolves attacking and defending cards.
    // Returns resolve result.
    fn eval_defense(&self) -> bool {
        // Early return on empty defender hand
        if self.defender_cards.is_empty() {
            return true;
        }

        // Must have same number of cards
        if self.attacker_cards.len() != self.defender_cards.len() {
            return false;
        }

        // Resolves a pair of attacking and defending card.
        // Returns true when defending succeeds.
        let resolve_pair = |a: &Card, b: &Card| -> bool {
            // If the cards are of same suit,
            // rank determines the outcome.
            if a.suit == b.suit {
                if card_rank_order(b.rank) > card_rank_order(a.rank) {
                    return true;
                }
            }

            // If suits are not equal,
            // check if B is a trump card.
            if b.suit == self.trump_card.suit {
                return true;
            }

            false
        };

        // Card order matters:
        // the attacking and defending cards must be lined up.
        //
        // TODO: automatic resolving without strict ordering?
        self.attacker_cards
            .iter()
            .zip(self.defender_cards.iter())
            .all(|(a, b)| resolve_pair(a, b))
    }

    // Clears playing table
    fn discard_table(&mut self) {
        self.discarded.extend(self.attacker_cards.drain(..));
        self.discarded.extend(self.defender_cards.drain(..));
    }
}

// Moska card rank ordering
fn card_rank_order(rank: Rank) -> usize {
    match rank {
        Rank::Two => 0,
        Rank::Three => 1,
        Rank::Four => 2,
        Rank::Five => 3,
        Rank::Six => 4,
        Rank::Seven => 5,
        Rank::Eight => 6,
        Rank::Nine => 7,
        Rank::Ten => 8,
        Rank::Jack => 9,
        Rank::Queen => 10,
        Rank::King => 11,
        Rank::Ace => 12,
        Rank::Joker => 13,
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Suit;

    use super::*;

    #[test]
    fn test_cards() {
        let mut game = Moska::new(2);
        game.new_round();

        assert_eq!(game.table.players[0].cards.len(), 6);
        assert_eq!(game.table.players[1].cards.len(), 6);

        // player 0 turn
        assert_eq!(game.table.player_index, 0);
        assert_eq!(game.state, State::PlayerAttacking);

        // play first card from hand
        game.player_action(PlayerAction::AddCard as usize, 0);
        assert_eq!(game.table.players[0].cards.len(), 5);
        assert_eq!(game.attacker_cards.len(), 1);

        // end attacking turn, should fill hand
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.table.players[0].cards.len(), 6);

        // player 1 turn
        assert_eq!(game.table.player_index, 1);
        assert_eq!(game.state, State::PlayerDefending);

        // play first card from hand
        game.player_action(PlayerAction::AddCard as usize, 0);
        assert_eq!(game.table.players[1].cards.len(), 5);
        assert_eq!(game.defender_cards.len(), 1);

        // take card back
        game.player_action(PlayerAction::TakeCard as usize, 0);
        assert_eq!(game.table.players[1].cards.len(), 6);
        assert_eq!(game.defender_cards.len(), 0);

        // fold turn
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.table.players[1].cards.len(), 7);
        assert_eq!(game.attacker_cards.len(), 0);

        // player 0 turn
        assert_eq!(game.table.player_index, 0);
        assert_eq!(game.state, State::PlayerAttacking);
    }

    #[test]
    fn test_attack() {
        let mut game = Moska::new(2);
        game.new_round();

        // empty table, should be illegal
        assert_eq!(game.eval_attack(), false);

        // add 2 of hearts, should be ok
        game.attacker_cards.push(Card::new(Suit::Hearts, Rank::Two));
        assert_eq!(game.eval_attack(), true);

        // add 3 of diamonds, should be illegal
        game.attacker_cards
            .push(Card::new(Suit::Diamonds, Rank::Three));
        assert_eq!(game.eval_attack(), false);
        game.attacker_cards.pop();

        // make a pair, should be ok
        game.attacker_cards
            .push(Card::new(Suit::Diamonds, Rank::Two));
        assert_eq!(game.eval_attack(), true);

        // add a second pair, should be ok
        game.attacker_cards
            .push(Card::new(Suit::Spades, Rank::Four));
        game.attacker_cards.push(Card::new(Suit::Clubs, Rank::Four));
        assert_eq!(game.attacker_cards.len(), 4);
        assert_eq!(game.eval_attack(), true);

        // pop last card out, should be illegal
        game.attacker_cards.pop();
        assert_eq!(game.attacker_cards.len(), 3);
        assert_eq!(game.eval_attack(), false);

        game.attacker_cards.pop();
        assert_eq!(game.eval_attack(), true);
    }

    #[test]
    fn test_defend() {
        let mut game = Moska::new(2);
        game.new_round();

        // set state, trump card
        game.state = State::PlayerDefending;
        game.trump_card = Card::new(Suit::Spades, Rank::Ace);

        // add single card
        game.attacker_cards.push(Card::new(Suit::Hearts, Rank::Two));
        game.defender_cards
            .push(Card::new(Suit::Hearts, Rank::Three));
        assert_eq!(game.eval_attack(), true);
        assert_eq!(game.eval_defense(), true);

        // make attack pair
        game.attacker_cards
            .push(Card::new(Suit::Diamonds, Rank::Two));
        assert_eq!(game.eval_attack(), true);
        assert_eq!(game.eval_defense(), false);

        // try to add wrong suit for defender
        game.defender_cards
            .push(Card::new(Suit::Clubs, Rank::Three));
        assert_eq!(game.eval_defense(), false);

        // replace correct suit for defender
        game.defender_cards.pop();
        game.defender_cards
            .push(Card::new(Suit::Diamonds, Rank::Three));
        assert_eq!(game.eval_defense(), true);

        // add a second pair
        game.attacker_cards.push(Card::new(Suit::Spades, Rank::Ten));
        game.attacker_cards.push(Card::new(Suit::Clubs, Rank::Ten));
        game.defender_cards
            .push(Card::new(Suit::Spades, Rank::Jack));
        game.defender_cards.push(Card::new(Suit::Clubs, Rank::Jack));
        assert_eq!(game.eval_defense(), true);

        // pop last card and try with lesser rank
        game.defender_cards.pop();
        game.defender_cards.push(Card::new(Suit::Clubs, Rank::Five));
        assert_eq!(game.eval_defense(), false);

        // try trump card
        game.defender_cards.pop();
        game.defender_cards
            .push(Card::new(Suit::Spades, Rank::Five));
        assert_eq!(game.eval_defense(), true);

        // clear table
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.attacker_cards.len(), 0);
        assert_eq!(game.defender_cards.len(), 0);
    }

    #[test]
    fn test_turns() {
        let mut game = Moska::new(3);
        game.new_round();
        game.trump_card = Card::new(Suit::Spades, Rank::Ace);

        // test initial state
        assert_eq!(game.state, State::PlayerAttacking);
        assert_eq!(game.table.turn, 0);
        assert_eq!(game.table.player_index, 0);

        // attempt to submit without cards on the table
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.state, State::PlayerAttacking);
        assert_eq!(game.table.player_index, 0);

        // add attacking card and end turn
        game.attacker_cards
            .push(Card::new(Suit::Hearts, Rank::Nine));
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.state, State::PlayerDefending);
        assert_eq!(game.table.turn, 1);
        assert_eq!(game.table.player_index, 1);

        // add defending card and submit
        game.defender_cards.push(Card::new(Suit::Hearts, Rank::Ace));
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.state, State::PlayerAttacking);
        assert_eq!(game.table.player_index, 1);

        // table should be clear
        assert!(game.attacker_cards.is_empty());
        assert!(game.defender_cards.is_empty());

        // add attacking cards and end turn
        game.attacker_cards
            .push(Card::new(Suit::Clubs, Rank::Seven));
        game.attacker_cards
            .push(Card::new(Suit::Diamonds, Rank::Seven));
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.state, State::PlayerDefending);
        assert_eq!(game.table.turn, 2);
        assert_eq!(game.table.player_index, 2);

        // submit without defending cards, taking the attacking cards
        game.player_action(PlayerAction::Submit as usize, 0);
        assert_eq!(game.table.players[2].cards.len(), 8);
        assert_eq!(game.state, State::PlayerAttacking);
        assert_eq!(game.table.turn, 3);
        assert_eq!(game.table.player_index, 0);
    }

    #[test]
    fn test_gameover() {
        let mut game = Moska::new(3);
        game.new_round();

        game.trump_card = Card::new(Suit::Spades, Rank::Ace);

        game.table.players[0].cards = vec![Card::new(Suit::Hearts, Rank::Two)];
        game.table.players[1].cards = vec![Card::new(Suit::Hearts, Rank::Three)];
        game.table.players[2].cards = vec![Card::new(Suit::Hearts, Rank::Four)];

        // pop all cards from deck
        while let Some(_) = game.table.deck.pop() {}

        assert_eq!(game.state, State::PlayerAttacking);
        game.player_action(PlayerAction::AddCard as usize, 0);
        game.player_action(PlayerAction::Submit as usize, 0);

        // player 0 should have no cards left
        assert!(game.table.players[0].cards.is_empty());

        assert_eq!(game.state, State::PlayerDefending);
        game.player_action(PlayerAction::AddCard as usize, 0);
        game.player_action(PlayerAction::Submit as usize, 0);

        // player 1 should have no cards left,
        // meaning player 2 has lost the game
        assert_eq!(game.state, State::GameOver);

        // try again with some cards present on the deck
        let mut game = Moska::new(3);
        game.new_round();

        game.trump_card = Card::new(Suit::Spades, Rank::Ace);

        game.table.players[0].cards = vec![Card::new(Suit::Hearts, Rank::Two)];
        game.table.players[1].cards = vec![Card::new(Suit::Hearts, Rank::Three)];
        game.table.players[2].cards = vec![Card::new(Suit::Hearts, Rank::Four)];

        // pop cards from the deck until 3 left
        while game.table.deck.count() > 3 {
            game.table.deck.pop();
        }

        assert_eq!(game.state, State::PlayerAttacking);
        game.player_action(PlayerAction::AddCard as usize, 0);
        assert_eq!(game.table.players[0].cards.len(), 0);
        assert_eq!(game.table.deck.count(), 3);

        game.player_action(PlayerAction::Submit as usize, 0);

        // player 0 should have all the cards from the deck
        assert_eq!(game.table.deck.count(), 0);
        assert_eq!(game.table.players[0].cards.len(), 3);

        assert_eq!(game.state, State::PlayerDefending);
        game.player_action(PlayerAction::AddCard as usize, 0);
        game.player_action(PlayerAction::Submit as usize, 0);

        assert_eq!(game.state, State::PlayerAttacking);
        game.player_action(PlayerAction::AddCard as usize, 0);
        game.player_action(PlayerAction::Submit as usize, 0);

        // player 0 should have lost the game
        assert_eq!(game.state, State::GameOver);
    }
}
