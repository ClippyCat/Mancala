use crate::mancala::*;

#[test]
fn test_i0() {
let mut state = GameState::default();
	move_rocks(&mut state, 0);
	let exp: [i8; BOARD_SIZE] = [0, 7, 7, 7, 7, 1, 7, 6, 6, 6, 6, 0];
	assert_eq!(state.board.pits, exp, "no dice!");
}

#[test]
fn test_init(){
	let b=init_board(6);
	let exp: [i8; BOARD_SIZE] = [6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 0];
	assert_eq!(b.pits, exp, "bad board");
}