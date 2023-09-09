use text_io::try_read;

const Board_Size: usize = 12;

// Define a struct for the game's board
pub struct Board {
    pits: [i8; Board_Size], //pits+mancalas
    activePlayer: u8,       // keep track of the active player
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
enum Winner {
    Player1,
    Player2,
    Tie,
}

// Define a struct for the game state
pub struct GameState {
    board: Board,
    player1: Player,
    player2: Player,
    status: GameStatus,
}
impl Default for GameState {
    fn default() -> GameState {
        GameState {
            player1: Player {
                name: "Player 1".to_string(),
                mancala_index: 0,
            },
            player2: Player {
                name: "Player 2".to_string(),
                mancala_index: 6,
            },
            status: GameStatus::InProgress,
            board: Board::default(),
        }
    }
}

// Define a function to initialize the game's board
pub fn init_board(numRocks: i8) -> Board {
    // Set up the board with the correct number of stones in each pit and mancala
    let pits = [numRocks; Board_Size];
    // Set player 1 as the active player
    let activePlayer = 1;
    Board { pits, activePlayer }
}

// Define a function to display the board
pub fn displayBoard(board: &Board) {
    // Print the second player's pits in reverse order
    for i in (0..(Board_Size / 2 - 1)).rev() {
        print!("{:3}", board.pits[i]);
    }
    // Print the second player's mancala
    println!("{:3}", board.pits[Board_Size / 2 - 1]);
    // Print the first player's mancala
    print!("{:3}", board.pits[Board_Size - 1]);
    // Print the first player's pits
    for i in (Board_Size / 2..Board_Size - 1).rev() {
        print!("{:3}", board.pits[i]);
    }
    println!();
}

// Define a function to handle player input
pub fn getMove() -> usize {
    loop {
        // Prompt the current player to select a pit to move stones from
        println!("Enter the index of the pit you want to move stones from:");
        // Use the text_io crate to read input from the terminal
        let input: Result<usize, _> = try_read!();
        // Check if the input was valid
        match input {
            Ok(pit_index) if (0..Board_Size / 2 - 1).contains(&pit_index) => {
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
pub fn moveRocks(game_state: &mut GameState, pitI: usize) {
    let mut i = pitI;
    {
        let board = &mut game_state.board;
        // Get the number of stones in the selected pit
        let numRocks = board.pits[pitI];
        // Set the selected pit to zero stones
        board.pits[pitI] = 0;
        // Distribute the stones around the board
        i = pitI;
        for _ in 0..numRocks {
            // Increment the i to move to the next pit
            i = (i + 1) % Board_Size;
            // Skip the opponent's mancala
            if i == game_state.player2.mancala_index && board.activePlayer == 1 {
                i = (i + 1) % Board_Size;
            } else if i == game_state.player1.mancala_index && board.activePlayer == 2 {
                i = (i + 1) % Board_Size;
            }
            // Add a stone to the current pit
            board.pits[i] += 1;
        }
    }
    // Check for captures
    captures(game_state, i);
    // Check the game status
    game_state.status = checkStatus(&mut game_state.board);
}

// Define a function to check for captures
pub fn captures(game_state: &mut GameState, lastI: usize) {
    let board = &mut game_state.board;
    let numPits = Board_Size / 2 - 1;
    let i = lastI;
    let activePlayer = board.activePlayer;
    let activeMancala = if activePlayer == 1 {
        game_state.player1.mancala_index
    } else {
        game_state.player2.mancala_index
    };
    if board.pits[i] == 1
        && i != game_state.player1.mancala_index
        && i != game_state.player2.mancala_index
    {
        let opposite = Board_Size - i;
        let canCapture = board.pits[opposite];
        if i < numPits && activePlayer == 1 || i > numPits && activePlayer == 2 {
            board.pits[opposite] = 0;
            board.pits[activeMancala] += canCapture;
        }
    }
}

// Define a function to check the game's status
pub fn checkStatus(board: &mut Board) -> GameStatus {
    let numPits = Board_Size / 2 - 1;
    let p1_pits: &[i8] = &board.pits[..numPits];
    let p2_pits: &[i8] = &board.pits[numPits + 1..Board_Size - 1];
    let p1_sum: i8 = p1_pits.iter().sum();
    let p2_sum: i8 = p2_pits.iter().sum();
    if p1_sum == 0 || p2_sum == 0 {
        // Game is over, distribute remaining stones to respective mancalas
        for i in 0..numPits {
            board.pits[numPits] += board.pits[i];
            board.pits[i] = 0;
        }
        for i in numPits + 1..Board_Size - 1 {
            board.pits[Board_Size - 1] += board.pits[i];
            board.pits[i] = 0;
        }
        // Determine winner based on number of stones in each player's mancala
        if board.pits[numPits] > board.pits[Board_Size - 1] {
            GameStatus::GameOver(Winner::Player1)
        } else if board.pits[numPits] < board.pits[Board_Size - 1] {
            return GameStatus::GameOver(Winner::Player2);
        } else {
            return GameStatus::GameOver(Winner::Tie);
        }
    } else {
        GameStatus::InProgress
    }
}

// Define a function to handle the game's flow
pub fn playGame(mut state: GameState) {
    // Play the game until it's over
    while state.status == GameStatus::InProgress {
        // Display the current board
        displayBoard(&state.board);
        // Get the current player's move
        let pit_index = getMove();
        // Move the rocks from the selected pit
        moveRocks(&mut state, pit_index);
        // Switch to the other player
        state.board.activePlayer = 3 - state.board.activePlayer;
    }
    // Print the final board
    displayBoard(&state.board);
    // Print the winner
    let p1_score = state.board.pits[state.player1.mancala_index];
    let p2_score = state.board.pits[state.player2.mancala_index];
    if p1_score > p2_score {
        println!("{} wins!", state.player1.name);
    } else if p2_score > p1_score {
        println!("{} wins!", state.player2.name);
    } else {
        println!("The game is a tie!");
    }
}
