use std::ops::Deref;

use crate::{
    game::moska::{card_cmp, find_pairs, State},
    Card, Moska,
};
use itertools::Itertools;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Action {
    pub action: usize,
    pub card_index: usize,
}

#[wasm_bindgen]
pub struct MoskaAI {
    player_index: usize,
}

#[wasm_bindgen]
impl MoskaAI {
    #[wasm_bindgen(constructor)]
    pub fn new(player_index: usize) -> Self {
        Self { player_index }
    }

    pub fn get_actions(&self, game: &Moska) -> Vec<Action> {
        let mut actions = vec![];

        //let resolve_card_index = |mut action: Action, previous_action: Option<&Action>| {
        //    if let Some(previous_action) = previous_action {
        //        if previous_action.card_index < action.card_index {
        //            action.card_index -= 1;
        //        }
        //    }
        //    action
        //};

        if game.table.player_index == self.player_index {
            if let Some(self_player) = game.table.players.get(self.player_index) {
                match game.state {
                    State::PlayerDefending => {
                        // Check if any combination yields successfull defense.
                        // Otherwise withdraw cards.
                        if let Some(cards) = self_player
                            .cards
                            .iter()
                            .permutations(game.attacker_cards.len())
                            .into_iter()
                            .find(|chunk| {
                                chunk
                                    .iter()
                                    .zip(&game.attacker_cards)
                                    .all(|(def, atk)| game.resolve_pair(atk, def))
                            })
                        {
                            // Play the cards to the table
                            for card in cards {
                                // Find index in hand
                                if let Some(card_index) = self_player.card_index(card) {
                                    // Add card to actions
                                    actions.push(Action {
                                        action: 1,
                                        card_index,
                                    });
                                }
                            }
                        }
                    }
                    State::PlayerAttacking => {
                        let trump_suit = game.trump_card.suit;
                        let max_cards = game.table.players[game.next_player()].cards.len();

                        // Order hand by card
                        let cards: Vec<&Card> = self_player
                            .cards
                            .iter()
                            .sorted_by(|a, b| card_cmp(*a, *b, trump_suit))
                            .collect::<Vec<&Card>>();

                        // Optimize best pair combination while keeping number of cards below or
                        // equal to card count in defenders hand.
                        'a: for i in (2..=max_cards).rev() {
                            for cards in cards.iter().combinations(i) {
                                let pairs = find_pairs(
                                    &cards.into_iter().map(Deref::deref).collect::<Vec<&Card>>(),
                                );
                                let num_cards: usize = pairs.iter().map(|v| v.len()).sum();
                                if num_cards <= max_cards {
                                    println!("Found pairs of len {num_cards}, {:?}", pairs);
                                    for pairs in pairs {
                                        for card in pairs {
                                            if let Some(card_index) = self_player.card_index(card) {
                                                actions.push(Action {
                                                    action: 1,
                                                    card_index,
                                                });
                                            }
                                        }
                                    }
                                    break 'a;
                                }
                            }
                        }

                        if actions.len() == 0 && cards.len() > 0 {
                            // Add first card from sorted cards
                            if let Some(card) = cards.get(0) {
                                if let Some(card_index) = self_player.card_index(card) {
                                    actions.push(Action {
                                        action: 1,
                                        card_index,
                                    });
                                }
                            }
                        }
                    }
                    _ => {
                        // nothing to do
                    }
                }
            }
        }

        println!(
            "{:?}",
            actions.iter().map(|a| a.card_index).collect::<Vec<usize>>()
        );

        // sort actions in descending order
        actions.sort_by(|a, b| b.card_index.cmp(&a.card_index));

        println!(
            "{:?}",
            actions.iter().map(|a| a.card_index).collect::<Vec<usize>>()
        );

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bots() {
        let mut game = Moska::new(2);
        game.new_round();

        let bots: Vec<MoskaAI> = game
            .table
            .players
            .iter()
            .map(|p| MoskaAI::new(p.id as usize))
            .collect();

        // bots should be able to play the game by themselves
        let mut iteration = 0;
        loop {
            iteration += 1;
            println!("iteration {iteration}");

            let i = game.table.player_index;
            println!("");
            println!("hand {:?}", game.table.players[i].cards);

            bots[i].get_actions(&game).into_iter().for_each(|action| {
                game.player_action(action.action, action.card_index);
            });
            println!("atk {:?}", game.attacker_cards);
            println!("def {:?}", game.defender_cards);
            println!("hand2 {:?}", game.table.players[i].cards);

            assert_eq!(
                game.valid()
                    || (game.state == State::PlayerAttacking && game.attacker_cards.is_empty()),
                true
            );
            game.player_action(3, 0);

            println!("");
            if game.state == State::GameOver {
                break;
            }
        }
    }
}
