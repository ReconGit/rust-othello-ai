use ansi_term::Colour;

use crate::{
    mcts::mcts_move,
    //minimax::minimax_move,
    othello::{Cell, Othello, State},
};

pub fn start_game() {
    println!("{}", Colour::Purple.paint("Welcome to Othello!"));

    let mut game = Othello::new();
    let mut round = 0;
    while [State::BlackTurn, State::WhiteTurn].contains(&game.state) {
        round += 1;
        println!("{}", Colour::Green.paint(format!("\nRound {}", round)));
        print_game_board(game.board);
        print_score(game.black_score, game.white_score);
        print_state(game.state);

        let position = if game.state == State::BlackTurn {
            random_move(&game, 0)
        } else {
            mcts_move(&game, 100)
        };
        game.make_move(position);
    }
    println!("\n     Game over!");
    print_game_board(game.board);
    print_score(game.black_score, game.white_score);
    print_state(game.state);
    println!();
}

fn print_game_board(game_board: [[Cell; 8]; 8]) {
    println!("   A B C D E F G H");
    for y in 0..8 {
        print!("{} |", y + 1);
        for x in 0..8 {
            let cell = game_board[y][x];
            match cell {
                Cell::Empty => print!(" "),
                Cell::Black => print!("{}", Colour::Black.paint("●")),
                Cell::White => print!("{}", Colour::White.paint("●")),
                Cell::Valid => print!("{}", Colour::Yellow.paint("●")),
            }
            print!("|");
        }
        println!();
    }
}

fn print_score(black_score: u8, white_score: u8) {
    println!("Black: {} | White: {}", black_score, white_score);
}

fn print_state(state: State) {
    let current_state = match state {
        // print colored
        State::BlackTurn => Colour::Black.paint("   Black's turn!"),
        State::WhiteTurn => Colour::White.paint("   White's turn!"),
        State::BlackWon => Colour::Black.paint("    Black won!"),
        State::WhiteWon => Colour::White.paint("    White won!"),
        State::Draw => Colour::Yellow.paint("      Draw!"),
    };
    println!("{}", current_state);
}

fn random_move(game: &Othello, _dummy: i8) -> (usize, usize) {
    let valid_moves = game.get_valid_moves();
    valid_moves[rand::random::<usize>() % valid_moves.len()]
}