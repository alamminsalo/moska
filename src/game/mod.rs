use crate::deck::Card;

pub mod moska;

// Game interface
pub trait Game {
    // Resets game
    fn reset(&mut self);

    // Deals cards
    fn deal(&mut self);

    // Starts a new round
    fn new_round(&mut self);

    // Attempts a player action.
    fn player_action(&mut self, action: usize, card_index: usize) -> bool;

    // Attempts to continue to next turn
    fn next_turn(&mut self) -> bool;

    // Checks if game is over and returns player index if true
    fn game_over(&self) -> Option<usize>;

    // Displays game state in text format.
    fn display(&self);

    fn player_cards(&self, player_index: usize) -> Vec<Card>;
}
