use text_io::try_read;
use std::cmp::Ordering;

pub const BOARD_SIZE: usize = 12;

// Define a struct for the game's board
pub struct Board {
	pub pits: [i8; BOARD_SIZE], //pits+mancalas
	pub active_player: u8,	   // keep track of the active player
}
impl Default for Board {
	fn default() -> Board {
		init_board(6)
	}
}

// Define a struct for the player information
pub struct Player {
	name: String,
	mancala_index: usize, // the index of the player's mancala on the board
}

// Define an enum for the game status
#[derive(PartialEq, Default)]
pub enum GameStatus {
	#[default]
	InProgress,
	GameOver(Winner),
}

// Define an enum to decide winner
#[derive(PartialEq)]
pub enum Winner {
	Player1,
	Player2,
	Tie,
}

// Define a struct for the game state
pub struct GameState {
	pub board: Board,
	pub player1: Player,
	pub player2: Player,
	pub status: GameStatus,
}
impl Default for GameState {
	fn default() -> GameState {
		GameState {
			player1: Player {
				name: "Player 1".to_string(),
				mancala_index: 5,
			},
			player2: Player {
				name: "Player 2".to_string(),
				mancala_index: 11,
			},
			status: GameStatus::InProgress,
			board: Board::default(),
		}
	}
}

// Define a function to initialize the game's board
pub fn init_board(num_rocks: i8) -> Board {
	// Set up the board with the correct number of stones in each pit and mancala
	let mut pits = [num_rocks; BOARD_SIZE];
pits[BOARD_SIZE-1]=0;
pits[BOARD_SIZE/2-1]=0;
	// Set player 1 as the active player
	let active_player = 1;
	Board { pits, active_player }
}

// Define a function to display the board
pub fn display_board(board: &Board) {

	print!("({})", board.pits[BOARD_SIZE - 1]);
	for i in (BOARD_SIZE / 2..BOARD_SIZE - 1).rev() {
		print!("{:3}", board.pits[i]);
	}

	println!();

	for i in 0..BOARD_SIZE / 2 - 1 {
		print!("{:3}", board.pits[i]);
	}
	println!("({})", board.pits[BOARD_SIZE / 2 - 1]);

}

// Define a function to handle player input
pub fn get_move() -> usize {
	loop {
		// Prompt the current player to select a pit to move stones from
		println!("Enter the index of the pit you want to move stones from:");
		// Use the text_io crate to read input from the terminal
		let input: Result<usize, _> = try_read!();
		// Check if the input was valid
		match input {
			Ok(pit_index) if (0..BOARD_SIZE / 2 - 1).contains(&pit_index) => {
				// Return the selected pit index if it's valid
				return pit_index;
			}
			_ => {
				// Print an error message if the input was invalid
				println!("Invalid input. Please enter a valid pit index.");
				continue;
			}
		}
	}
}

// Define a function to distribute the stones
pub fn move_rocks(game_state: &mut GameState, pit_i: usize) {
	let mut i = pit_i;
	{
		let board = &mut game_state.board;
		// Get the number of stones in the selected pit
		let num_rocks = board.pits[pit_i];
		// Set the selected pit to zero stones
		board.pits[pit_i] = 0;
		// Distribute the stones around the board
		for _ in 0..num_rocks {
			// Increment the i to move to the next pit
			i = (i + 1) % BOARD_SIZE;
			// Skip the opponent's mancala
			if (i == game_state.player2.mancala_index && board.active_player == 1) || (i == game_state.player1.mancala_index && board.active_player == 2){
				i = (i + 1) % BOARD_SIZE;
			} 

			// Add a stone to the current pit
			board.pits[i] += 1;
		}
	}
	// Check for captures
	captures(game_state, i);
	// Check the game status
	game_state.status = check_status(&mut game_state.board);
}

// Define a function to check for captures
pub fn captures(game_state: &mut GameState, last_i: usize) {
	let board = &mut game_state.board;
	let num_pits = BOARD_SIZE / 2 - 1;
	let i = last_i;
	let active_player = board.active_player;
	let active_mancala = if active_player == 1 {
		game_state.player1.mancala_index
	} else {
		game_state.player2.mancala_index
	};
	if board.pits[i] == 1
		&& i != game_state.player1.mancala_index
		&& i != game_state.player2.mancala_index
	{
		let opposite = BOARD_SIZE - i;
		let can_capture = board.pits[opposite];
		if i < num_pits && active_player == 1 || i > num_pits && active_player == 2 {
			board.pits[opposite] = 0;
			board.pits[active_mancala] += can_capture;
		}
	}
}

// Define a function to check the game's status
pub fn check_status(board: &mut Board) -> GameStatus {
	let num_pits = BOARD_SIZE / 2 - 1;
	let p1_pits: &[i8] = &board.pits[..num_pits];
	let p2_pits: &[i8] = &board.pits[num_pits + 1..BOARD_SIZE - 1];
	let p1_sum: i8 = p1_pits.iter().sum();
	let p2_sum: i8 = p2_pits.iter().sum();
	if p1_sum == 0 || p2_sum == 0 {
		// Game is over, distribute remaining stones to respective mancalas
		for i in 0..num_pits {
			board.pits[num_pits] += board.pits[i];
			board.pits[i] = 0;
		}
		for i in num_pits + 1..BOARD_SIZE - 1 {
			board.pits[BOARD_SIZE - 1] += board.pits[i];
			board.pits[i] = 0;
		}
		// Determine winner based on number of stones in each player's mancala
		match board.pits[num_pits].cmp(&board.pits[BOARD_SIZE - 1]) {
			Ordering::Less=> GameStatus::GameOver(Winner::Player1),
			Ordering::Greater=> GameStatus::GameOver(Winner::Player2),
			Ordering::Equal=> GameStatus::GameOver(Winner::Tie)
		}
	}
	else {
		GameStatus::InProgress
	}
}

// Define a function to handle the game's flow
pub fn play_game(mut state: GameState) {
	// Play the game until it's over
	while state.status == GameStatus::InProgress {
		// Display the current board
		display_board(&state.board);
		// Get the current player's move
		let pit_index = get_move();
		// Move the rocks from the selected pit
		move_rocks(&mut state, pit_index);
		// Switch to the other player
		state.board.active_player = 3 - state.board.active_player;
	}
	// Print the final board
	display_board(&state.board);
	// Print the winner
	let p1_score = state.board.pits[state.player1.mancala_index];
	let p2_score = state.board.pits[state.player2.mancala_index];
	match p1_score.cmp(&p2_score) {
		Ordering::Greater=> println!("{} wins!", state.player1.name),
		Ordering::Less=> println!("{} wins!", state.player2.name),
		Ordering::Equal=> println!("The game is a tie!")
	}
}
