const Board_Size: usize = 12;

// Define a struct for the game's board
struct Board {
    pits: [i8; Board_Size], //pits+mancalas
    activePlayer: u8,       // keep track of the active player
}

// Define a struct for the player information
struct Player {
    name: String,
    mancala_index: usize, // the index of the player's mancala on the board
}

// Define an enum for the game status
enum GameStatus {
    InProgress,
    GameOver(Winner),
}

// Define an enum to decide winner
enum Winner {
    player1,
    player2,
    tie,
}

// Define a struct for the game state
struct GameState {
    board: Board,
    player1: Player,
    player2: Player,
    status: GameStatus,
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
        let input: Result<usize, _> = read!();
        // Check if the input was valid
        match input {
            Ok(pit_index) if pit_index >= 0 && pit_index < Board_Size as usize / 2 - 1 => {
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
pub fn moveRocks(board: &mut Board, pitI: usize) {
    // Get the number of stones in the selected pit
    let numRocks = board.pits[pitI];
    // Set the selected pit to zero stones
    board.pits[pitI] = 0;
    // Distribute the stones around the board
    let mut i = pitI;
    for _ in 0..numRocks {
        // Increment the i to move to the next pit
        i = (i + 1) % Board_Size as usize;
        // Skip the opponent's mancala
        if i == game_state.player2.mancala_index && board.activePlayer == 1 {
            i = (i + 1) % Board_Size as usize;
        } else if i == game_state.player1.mancala_index && board.activePlayer == 2 {
            i = (i + 1) % Board_Size as usize;
        }
        // Add a stone to the current pit
        board.pits[i] += 1;
    }
    // Check for captures
    captures(board, i);
    // Check the game status
    board.status = checkStatus(board);
}

// Define a function to check for captures
pub fn captures(board: &mut Board, lastI: usize) {
    let numPits = Board_Size as usize / 2 - 1;
    let mut i = lastI;
    let activePlayer = board.activePlayer;
    if board.pits[i] == 1
        && i != game_state.player1.mancala_index
        && i != game_state.player2.mancala_index
    {
        let opposite = Board_Size - i;
        let canCapture = board.pits[opposite];
        if i < numPits && activePlayer == 1 || i > numPits && activePlayer == 2 {
            board.pits[opposite] = 0;
            board.pits[activePlayer.mancala_index()] += canCapture;
        }
    }
}

// Define a function to check the game's status
pub fn checkStatus(board: &Board) -> GameStatus {
    let numPits = Board_Size as usize / 2 - 1;
    let p1_pits: &[i8] = &board.pits[..numPits];
    let p2_pits: &[i8] = &board.pits[numPits + 1..Board_Size as usize - 1];
    let p1_sum: i8 = p1_pits.iter().sum();
    let p2_sum: i8 = p2_pits.iter().sum();
    if p1_sum == 0 || p2_sum == 0 {
        // Game is over, distribute remaining stones to respective mancalas
        for i in 0..numPits {
            board.pits[numPits] += board.pits[i];
            board.pits[i] = 0;
        }
        for i in numPits + 1..Board_Size as usize - 1 {
            board.pits[Board_Size as usize - 1] += board.pits[i];
            board.pits[i] = 0;
        }
        // Determine winner based on number of stones in each player's mancala
        if board.pits[numPits] > board.pits[Board_Size as usize - 1] {
            return GameStatus::GameOver(player1);
        } else if board.pits[numPits] < board.pits[Board_Size as usize - 1] {
            return GameStatus::GameOver(player2);
        } else {
            return GameStatus::GameOver(tie);
        }
    } else {
        return GameStatus::InProgress;
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
        moveRocks(&mut state.board, pit_index);
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
