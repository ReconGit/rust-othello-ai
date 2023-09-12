use crate::othello::{Cell, Othello, State};

const REWARDS: [[i32; 8]; 8] = [
    [80, -20, 20, 10, 10, 20, -20, 80],
    [-20, -40, -10, -10, -10, -10, -40, -20],
    [20, -10, 10, 0, 0, 10, -10, 20],
    [10, -10, 0, 5, 5, 0, -10, 10],
    [10, -10, 0, 5, 5, 0, -10, 10],
    [20, -10, 10, 0, 0, 10, -10, 20],
    [-20, -40, -10, -10, -10, -10, -40, -20],
    [80, -20, 20, 10, 10, 20, -20, 80],
];

pub fn minimax_move(game: &Othello, mut depth: i16) -> (usize, usize) {
    let possible_moves = game.get_valid_moves();
    if possible_moves.is_empty() {
        panic!("Minimax: No valid moves!");
    }
    // if only one move, return it
    if possible_moves.len() == 1 {
        return possible_moves[0];
    }
    // if round is less than n, use random move
    let round = get_round(game.board);
    if round < 3 {
        return possible_moves[rand::random::<usize>() % possible_moves.len()];
    }
    // iterate depth based on round
    if round >= 50 {
        depth += 10;
    } else if round > 40 {
        depth += 2;
    } else if round > 30 {
        depth += 1;
    }

    let my_turn = game.state;
    let mut best_move = possible_moves[0];
    let mut best_value = std::i32::MIN;
    for move_ in possible_moves {
        let mut simulation = game.clone();
        simulation.make_move(move_);
        let value = minimax(&simulation, my_turn, depth - 1, std::i32::MIN, std::i32::MAX);
        if value > best_value {
            best_move = move_;
            best_value = value;
        }
        if value >= 300 {
            break;
        }
    }
    best_move
}

fn minimax(game: &Othello, my_turn: State, depth: i16, mut alpha: i32, mut beta: i32) -> i32 {
    let state = game.state;
    if state == State::BlackWon {
        return if State::BlackTurn == my_turn { 300 } else { -300 };
    } else if state == State::WhiteWon {
        return if State::WhiteTurn == my_turn { 300 } else { -300 };
    } else if state == State::Draw {
        return 0;
    }
    if depth == 0 {
        return evaluate_board(game.board, my_turn);
    }
    let possible_moves = game.get_valid_moves();
    // if state is my turn, best_value is MIN, else MAX
    let mut best_value = if state == my_turn { std::i32::MIN } else { std::i32::MAX };
    for move_ in possible_moves {
        let mut simulation = game.clone();
        simulation.make_move(move_);
        let value = minimax(&simulation, my_turn, depth - 1, alpha, beta);
        // alpha-beta pruning
        if state == my_turn {
            best_value = std::cmp::max(best_value, value);
            alpha = std::cmp::max(best_value, alpha);
        } else {
            best_value = std::cmp::min(best_value, value);
            beta = std::cmp::min(best_value, beta);
        }
        if alpha >= beta {
            break;
        }
    }
    best_value
}

fn evaluate_board(game_board: [[Cell; 8]; 8], my_turn: State) -> i32 {
    let mut score = 0;
    for y in 0..8 {
        for x in 0..8 {
            let cell = game_board[y][x];
            if cell == Cell::Black {
                score += if my_turn == State::BlackTurn { REWARDS[y][x] } else { -REWARDS[y][x] };
            } else if cell == Cell::White {
                score += if my_turn == State::WhiteTurn { REWARDS[y][x] } else { -REWARDS[y][x] };
            }
        }
    }
    score
}

fn get_round(game_board: [[Cell; 8]; 8]) -> i8 {
    let mut round = -3;
    for y in 0..8 {
        for x in 0..8 {
            if [Cell::Black, Cell::White].contains(&game_board[y][x]) {
                round += 1;
            }
        }
    }
    round
}
