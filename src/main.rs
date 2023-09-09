mod mancala;
mod tests;
use mancala::*;

fn main() {
    let status = GameState::default();
    playGame(status);
}
