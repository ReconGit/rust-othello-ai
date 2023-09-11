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
        println!("{}", Colour::Green.paint(format!("\n      Round {}", round)));
        print_board(game.board);
        print_score(game.black_score, game.white_score);
        print_state(game.state);

        let position = if game.state == State::BlackTurn {
            mcts_move(&game, 10)
        } else {
            mcts_move(&game, 10)
        };
        println!("      Move: {}{}", (position.0 as u8 + 65) as char, position.1 + 1);
        game.make_move(position);
    }
    println!("\n     Game Over!");
    print_board(game.board);
    print_score(game.black_score, game.white_score);
    print_state(game.state);
    println!();
}

fn print_board(game_board: [[Cell; 8]; 8]) {
    println!("   A B C D E F G H");
    for y in 0..8 {
        print!("{} |", y + 1);
        for x in 0..8 {
            let cell = game_board[y][x];
            match cell {
                Cell::Empty => print!(" "),
                Cell::Black => print!("{}", Colour::Black.paint("●")),
                Cell::White => print!("{}", Colour::White.paint("●")),
                Cell::Valid => print!("{}", Colour::Yellow.paint("*")),
            }
            print!("|");
        }
        println!();
    }
}

fn print_score(black_score: u8, white_score: u8) {
    print!("{}", Colour::Black.paint(format!("Black: {}", black_score)));
    println!(" | White: {}", white_score);
}

fn print_state(state: State) {
    let current_state = match state {
        // print colored
        State::BlackTurn => Colour::Black.paint("     BLACK turn"),
        State::WhiteTurn => Colour::White.paint("     WHITE turn"),
        State::BlackWon =>  Colour::Black.paint("     BLACK won"),
        State::WhiteWon =>  Colour::White.paint("     WHITE won"),
        State::Draw =>     Colour::Yellow.paint("        DRAW"),
    };
    println!("{}", current_state);
}
