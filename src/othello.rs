#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
    Valid,
}

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    BlackTurn,
    WhiteTurn,
    BlackWon,
    WhiteWon,
    Draw,
}

#[derive(Clone)]
pub struct Othello {
    pub black_score: u8,
    pub white_score: u8,
    pub board: [[Cell; 8]; 8],
    pub state: State,
}

impl Othello {
    pub fn new() -> Othello {
        // initialize board
        let mut board = [[Cell::Empty; 8]; 8];
        board[3][3] = Cell::White;
        board[3][4] = Cell::Black;
        board[4][3] = Cell::Black;
        board[4][4] = Cell::White;
        board[2][3] = Cell::Valid;
        board[3][2] = Cell::Valid;
        board[4][5] = Cell::Valid;
        board[5][4] = Cell::Valid;
        // initialize starting state
        Othello {
            black_score: 2,
            white_score: 2,
            board,
            state: State::BlackTurn,
        }
    }

    pub fn make_move(&mut self, position: (usize, usize)) {
        // sanity checks
        let reverse = match self.state {
            State::BlackTurn => Cell::Black,
            State::WhiteTurn => Cell::White,
            _ => panic!("Can't make move: Game is over!"),
        };
        if self.board[position.1][position.0] != Cell::Valid {
            panic!("Can't make move: Invalid position!");
        }
        // reverse cells
        self.board[position.1][position.0] = reverse;
        for cell in self.flipped_cells(position) {
            self.board[cell.1][cell.0] = reverse;
        }
        // update game state
        self.update_state();
    }

    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        // return empty vector if game is over
        if ![State::BlackTurn, State::WhiteTurn].contains(&self.state) {
            return Vec::new();
        }
        let mut valid_moves = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x] == Cell::Valid {
                    valid_moves.push((x, y));
                }
            }
        }
        valid_moves
    }

    fn update_state(&mut self) -> () {
        // update score
        self.black_score = self.board.iter().flatten().filter(|&x| *x == Cell::Black).count() as u8;
        self.white_score = self.board.iter().flatten().filter(|&x| *x == Cell::White).count() as u8;
        // check if game is over
        if self.is_board_full() || self.black_score == 0 || self.white_score == 0 {
            self.decide_winner();
            return;
        }
        // switch turns and update valid moves
        if self.state == State::BlackTurn {
            self.state = State::WhiteTurn;
        } else {
            self.state = State::BlackTurn;
        }
        self.update_valid_cells();
        // if no valid moves, switch turns again
        if self.get_valid_moves().is_empty() {
            if self.state == State::BlackTurn {
                self.state = State::WhiteTurn;
            } else {
                self.state = State::BlackTurn;
            }
            self.update_valid_cells();
            // if still no valid moves, game is over
            if self.get_valid_moves().is_empty() {
                self.decide_winner();
            }
        }
    }

    fn is_board_full(&self) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if [Cell::Empty, Cell::Valid].contains(&self.board[y][x]) {
                    return false;
                }
            }
        }
        true
    }

    fn decide_winner(&mut self) -> () {
        if self.black_score > self.white_score {
            self.state = State::BlackWon;
        } else if self.black_score < self.white_score {
            self.state = State::WhiteWon;
        } else {
            self.state = State::Draw;
        }
    }

    fn update_valid_cells(&mut self) -> () {
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x] == Cell::Valid {
                    self.board[y][x] = Cell::Empty;
                }
                if self.board[y][x] == Cell::Empty && !self.flipped_cells((x, y)).is_empty() {
                    self.board[y][x] = Cell::Valid;
                }
            }
        }
    }

    fn flipped_cells(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let player;
        let opponent;
        if self.state == State::BlackTurn {
            player = Cell::Black;
            opponent = Cell::White;
        } else {
            player = Cell::White;
            opponent = Cell::Black;
        }
        // check in all directions
        let directions = [(0, 1), (0, -1), (1, 0), (1, 1), (1, -1), (-1, 0), (-1, 1), (-1, -1)];
        let mut flipped = Vec::new();
        for direction in directions {
            flipped.append(&mut self.flipped_cells_in_directions(position.0, position.1, direction.0, direction.1, player, opponent));
        }
        flipped
    }

    fn flipped_cells_in_directions(&self, mut x: usize, mut y: usize, dx: i8, dy: i8, player: Cell, opponent: Cell) -> Vec<(usize, usize)> {
        let mut flipped = Vec::new();
        x = (x as i8 + dx) as usize;
        y = (y as i8 + dy) as usize;
        while (0..8).contains(&x) && (0..8).contains(&y) && self.board[y][x] == opponent {
            flipped.push((x, y));
            x = (x as i8 + dx) as usize;
            y = (y as i8 + dy) as usize;
        }
        if !((0..8).contains(&x) && (0..8).contains(&y)) || self.board[y][x] != player {
            return Vec::new();
        }
        flipped
    }
}
