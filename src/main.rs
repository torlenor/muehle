use std::fmt;
use std::io::{self, BufRead};

#[derive(PartialEq)]
enum GameState {
    Phase1,
    Phase2,
    Phase3,
    EndGame,
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    None,
    One,
    Two,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Player::None => write!(f, "None (Draw)"),
            Player::One => write!(f, "Player One"),
            Player::Two => write!(f, "Player Two"),
        }
    }
}

struct Board {
    allowed_moves: Vec<Vec<u8>>,
    possible_mills: Vec<Vec<u8>>,
    stones: Vec<Player>,
}

impl Board {
    // Create a new Muehle game board
    fn new() -> Board {
        // Define all allowed moves on the board
        let mut allowed_moves: Vec<Vec<u8>> = Vec::new();
        allowed_moves.push(vec![1, 9]);
        allowed_moves.push(vec![0, 2, 4]);
        allowed_moves.push(vec![1, 14]);
        allowed_moves.push(vec![10, 4]);
        allowed_moves.push(vec![3, 5, 7, 1]);
        allowed_moves.push(vec![4, 13]);
        allowed_moves.push(vec![11, 7]);
        allowed_moves.push(vec![6, 4, 8]);
        allowed_moves.push(vec![7, 12]);
        allowed_moves.push(vec![0, 21]);
        allowed_moves.push(vec![9, 3, 18, 11]);
        allowed_moves.push(vec![6, 10, 15]);
        allowed_moves.push(vec![8, 13, 17]);
        allowed_moves.push(vec![12, 5, 14, 20]);
        allowed_moves.push(vec![13, 2, 23]);
        allowed_moves.push(vec![11, 16]);
        allowed_moves.push(vec![15, 19, 17]);
        allowed_moves.push(vec![16, 12]);
        allowed_moves.push(vec![10, 19]);
        allowed_moves.push(vec![18, 20]);
        allowed_moves.push(vec![19, 13]);
        allowed_moves.push(vec![9, 22]);
        allowed_moves.push(vec![21, 19, 23]);
        allowed_moves.push(vec![22, 14]);

        let mut possible_mills: Vec<Vec<u8>> = Vec::new();
        possible_mills.push(vec![0, 1, 2]);
        possible_mills.push(vec![3, 4, 5]);
        possible_mills.push(vec![6,7,8]);
        possible_mills.push(vec![15,16,17]);
        possible_mills.push(vec![18,19,20]);
        possible_mills.push(vec![21,22,23]);
        possible_mills.push(vec![0,9,21]);
        possible_mills.push(vec![3,10,18]);
        possible_mills.push(vec![6,11,15]);
        possible_mills.push(vec![8,12,17]);
        possible_mills.push(vec![5,13,20]);
        possible_mills.push(vec![2,14,23]);
        possible_mills.push(vec![1,4,7]);
        possible_mills.push(vec![12,13,14]);
        possible_mills.push(vec![16,19,22]);
        possible_mills.push(vec![9,10,11]);
        
        assert_eq!(16, possible_mills.len());

        let stones: Vec<Player> = vec![Player::None; 24];

        Board {
            allowed_moves,
            possible_mills,
            stones,
        }
    }

    fn allowed_move(&self, x: u8, y: u8) -> bool {
        self.allowed_moves[usize::from(x)].iter().any(|&i| i == y)
    }

    // Place a new stone on the board at the given location x for player p.
    // Returns true if it was successful.
    fn place(&mut self, x: u8, p: Player) -> bool {
        if usize::from(x) >= self.stones.len() {
            return false;
        }
        if self.is_occupied(x) {
            return false;
        }
        self.stones[usize::from(x)] = p;
        true
    }

    // Move a stone from x to y
    // Returns true if it was successful.
    fn mv(&mut self, x: u8, y: u8, p: Player) -> bool {
        if !self.allowed_move(x, y) {
            return false;
        }

        self.jump(x, y, p)
    }

    // Jump with a stone from x to y
    // Returns true if it was successful.
    fn jump(&mut self, x: u8, y: u8, p: Player) -> bool {
        if usize::from(x) >= self.stones.len()
            || usize::from(y) >= self.stones.len()
            || self.is_occupied(y)
            || self.stones[usize::from(x)] != p
        {
            return false;
        }

        self.stones[usize::from(x)] = Player::None;
        self.stones[usize::from(y)] = p;

        true
    }

    // Returns true of the given location (x,y) is occupied.
    fn is_occupied(&self, x: u8) -> bool {
        self.stones[usize::from(x)] != Player::None
    }

    // Number of stones for the given player on the board
    fn number_of_stones(&self, p: Player) -> u8 {
        let mut cnt: u8 = 0;
        for player in &self.stones {
            if *player == p {
                cnt += 1;
            }
        }
        cnt
    }

    // Returns the player which currently has a mill and the mills.
    // Returns Player::None if no player has a mill.
    fn get_mill(&self) -> (Player, Option<Vec<u8>>) {
        (Player::None, None)
    }

    // Returns true if the given stone position is part of a mill.
    fn is_part_of_mill(&self, x: u8) -> bool {
        let player = self.stones[usize::from(x)];
        if player ==  Player::None {
            return false;
        }

        let mills_iter = self.possible_mills.iter();
        for val in mills_iter {
            if val.iter().any(|&i| i == x) {
                for stone in val.iter() {
                    if self.stones[usize::from(*stone)] != player {
                        return false;
                    }
                }
                return true;
            }
        }

        false
    }

    fn piece(&self, x: u8) -> &str {
        match self.stones[usize::from(x)] {
            Player::None => "0",
            Player::One => "1",
            Player::Two => "2",
        }
    }

    fn print(&self) {
        println!("Current board state:");
        println!(
            "{}----------{}----------{}",
            self.piece(0),
            self.piece(1),
            self.piece(2)
        );
        println!("|          |          |");
        println!(
            "|   {}------{}------{}   |",
            self.piece(3),
            self.piece(4),
            self.piece(5)
        );
        println!("|   |      |      |   |");
        println!(
            "|   |   {}--{}--{}   |   |",
            self.piece(6),
            self.piece(7),
            self.piece(8)
        );
        println!("|   |   |     |   |   |");
        println!(
            "{}---{}---{}     {}---{}---{}",
            self.piece(9),
            self.piece(10),
            self.piece(11),
            self.piece(12),
            self.piece(13),
            self.piece(14)
        );
        println!("|   |   |     |   |   |");
        println!(
            "|   |   {}--{}--{}   |   |",
            self.piece(15),
            self.piece(16),
            self.piece(17)
        );
        println!("|   |      |      |   |");
        println!(
            "|   {}------{}------{}   |",
            self.piece(18),
            self.piece(19),
            self.piece(20)
        );
        println!("|          |          |");
        println!(
            "{}----------{}----------{}",
            self.piece(21),
            self.piece(22),
            self.piece(23)
        );
        println!(
            "Number of stones on board {}: {}",
            Player::One,
            self.number_of_stones(Player::One)
        );
        println!(
            "Number of stones on board {}: {}",
            Player::Two,
            self.number_of_stones(Player::Two)
        )
    }
}

fn main() {
    println!("Welcome to Muehle!");

    println!("Creating Board...");
    let mut board = Board::new();

    let mut current_game_state = GameState::Phase1;
    let mut next_move = Player::One;
    let mut winner = Player::None;

    println!("Game starts...");
    println!();

    while current_game_state != GameState::EndGame {
        board.print();
        println!();

        match current_game_state {
            GameState::Phase1 => {
                println!("Phase 1: You can place stones freely on the board (enter a number indicating where you want to place your stone).");

                println!("{} where do you want to place your next stone?", next_move);
                let mut move_str = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut move_str).unwrap();

                match move_str.trim().parse::<u8>() {
                    Ok(pos) => {
                        let ok = board.place(pos, next_move);
                        if !ok {
                            println!("\nInvalid move. Try again.\n");
                            continue;
                        }

                        if board.is_part_of_mill(pos) {
                            println!(
                                "{} You have a MILL! Pass a location were a stone of the opponent shall be removed:",
                                next_move
                            );
                        }
                    }
                    Err(e) => {
                        println!("\nInvalid move ({}). Try again.\n", e);
                        continue;
                    }
                }

                if board.number_of_stones(Player::One) == 9
                    && board.number_of_stones(Player::Two) == 9
                {
                    current_game_state = GameState::Phase2;
                }
            }
            GameState::Phase2 => {
                println!("Phase 2: You can move one of your stones to an adjacent empty spot.");
                println!(
                    "{} what is your next move? (Format: Type 0,1 to move stone 0 to 1)",
                    next_move
                );

                let mut move_str = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut move_str).unwrap();
                let split_str: Vec<&str> = move_str.split(',').collect();
                if split_str.len() != 2 {
                    println!("\nInvalid move. Format is 'x,y'. Try again.\n");
                    continue;
                }

                let origin: u8;
                let dst: u8;
                match split_str[0].trim().parse::<u8>() {
                    Ok(pos) => {
                        origin = pos;
                    }
                    Err(e) => {
                        println!("\nInvalid origin ({}). Try again.\n", e);
                        continue;
                    }
                }

                match split_str[1].trim().parse::<u8>() {
                    Ok(pos) => {
                        dst = pos;
                    }
                    Err(e) => {
                        println!("\nInvalid destination ({}). Try again.\n", e);
                        continue;
                    }
                }

                if !board.mv(origin, dst, next_move) {
                    println!("\nInvalid move. Try again.\n");
                    continue;
                }

                if board.is_part_of_mill(dst) {
                    println!(
                        "{} You have a MILL! Pass a location were a stone of the opponent shall be removed:",
                        next_move
                    );
                }

                if board.number_of_stones(Player::One) == 3
                    || board.number_of_stones(Player::Two) == 3
                {
                    current_game_state = GameState::Phase3;
                }
            }
            GameState::Phase3 => {
                println!("Phase 3: Players with only 3 stones left can jump freely on the board.");
                if board.number_of_stones(next_move) == 3 {
                    println!(
                        "{} what is your next move? (Format: Type 0,10 to jump with stone 0 to 10)",
                        next_move
                    );
                } else {
                    println!(
                        "{} what is your next move? (Format: Type 0,1 to move stone 0 to 1)",
                        next_move
                    );
                }

                let mut move_str = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut move_str).unwrap();
                let split_str: Vec<&str> = move_str.split(',').collect();
                if split_str.len() != 2 {
                    println!("\nInvalid move. Format is 'x,y'. Try again.\n");
                    continue;
                }

                let origin: u8;
                let dst: u8;
                match split_str[0].trim().parse::<u8>() {
                    Ok(pos) => {
                        origin = pos;
                    }
                    Err(e) => {
                        println!("\nInvalid origin ({}). Try again.\n", e);
                        continue;
                    }
                }

                match split_str[1].trim().parse::<u8>() {
                    Ok(pos) => {
                        dst = pos;
                    }
                    Err(e) => {
                        println!("\nInvalid destination ({}). Try again.\n", e);
                        continue;
                    }
                }

                if board.number_of_stones(next_move) == 3 {
                    if !board.jump(origin, dst, next_move) {
                        println!("\nInvalid move. Try again.\n");
                        continue;
                    }
                } else if !board.mv(origin, dst, next_move) {
                    println!("\nInvalid move. Try again.\n");
                    continue;
                }

                if board.is_part_of_mill(dst) {
                    println!(
                        "{} You have a MILL! Pass a location were a stone of the opponent shall be removed:",
                        next_move
                    );
                }

                if board.number_of_stones(Player::One) < 3 {
                    winner = Player::Two;
                    current_game_state = GameState::EndGame;
                } else if board.number_of_stones(Player::Two) < 3 {
                    winner = Player::One;
                    current_game_state = GameState::EndGame;
                }
            }
            GameState::EndGame => {}
        }

        if next_move == Player::One {
            next_move = Player::Two;
        } else {
            next_move = Player::One;
        }

        println!();
    }

    println!("Game finished. Winner is {}", winner);
}
