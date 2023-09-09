mod mancala;
mod tests;
use mancala::*;

fn main() {
    let status = GameState::default();
    play_game(status);
}
