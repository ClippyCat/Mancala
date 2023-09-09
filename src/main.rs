mod mancala;
use mancala::*;

fn main() {
    let status = GameState::default();
    playGame(status);
}
