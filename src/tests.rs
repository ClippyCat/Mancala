use crate::mancala::*;

#[test]
fn testI0() {
let mut state = GameState::default();
	moveRocks(&mut state, 0);
	let exp: [i8; Board_Size] = [0, 7, 7, 7, 7, 1, 7, 6, 6, 6, 6, 0];
	assert_eq!(state.board.pits, exp, "no dice!");
}

#[test]
fn testInit(){
let b=init_board(6);
	let exp: [i8; Board_Size] = [6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 0];
	assert_eq!(b.pits, exp, "bad board");
}