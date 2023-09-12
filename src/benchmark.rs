use ansi_term::Colour;
use std::{io::Write, time::Instant};

use crate::{
    mcts::mcts_move,
    minimax::minimax_move,
    othello::{Othello, State},
};

const GAMES: i16 = 10;
const MINIMAX_DEPTH: i16 = 1;
const MCTS_ITERATIONS: i16 = 100;

pub fn run_benchmarks() {
    println!("{}", Colour::Purple.paint("Running benchmarks..."));
    let start_time = Instant::now();

    println!("{}", Colour::Blue.paint("\nRandom vs Random"));
    benchmark_game(random_move, random_move, 0, 0);

    println!("{}", Colour::Blue.paint("\nBLACK Minimax vs WHITE Random"));
    benchmark_game(minimax_move, random_move, MINIMAX_DEPTH, 0);

    println!("{}", Colour::Blue.paint("\nWHITE Minimax vs BLACK Random"));
    benchmark_game(random_move, minimax_move, 0, MINIMAX_DEPTH);

    println!("{}", Colour::Blue.paint("\nMinimax vs Minimax"));
    benchmark_game(minimax_move, minimax_move, MINIMAX_DEPTH, MINIMAX_DEPTH);

    println!("{}", Colour::Blue.paint("\nBLACK MCTS vs WHITE Random"));
    benchmark_game(mcts_move, random_move, MCTS_ITERATIONS, 0);

    println!("{}", Colour::Blue.paint("\nWHITE MCTS vs BLACK Random"));
    benchmark_game(random_move, mcts_move, 0, MCTS_ITERATIONS);

    println!("{}", Colour::Blue.paint("\nMCTS vs MCTS"));
    benchmark_game(mcts_move, mcts_move, MCTS_ITERATIONS, MCTS_ITERATIONS);

    println!("{}", Colour::Blue.paint("\nBLACK Minimax vs WHITE MCTS"));
    benchmark_game(minimax_move, mcts_move, MINIMAX_DEPTH, MCTS_ITERATIONS);

    println!("{}", Colour::Blue.paint("\nBLACK MCTS vs WHITE Minimax"));
    benchmark_game(mcts_move, minimax_move, MCTS_ITERATIONS, MINIMAX_DEPTH);

    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("{}", Colour::Purple.paint(format!("\nTotal elapsed time: {:.2}s", elapsed_time)));
}

fn benchmark_game(
    black_ai: fn(&Othello, i16) -> (usize, usize),
    white_ai: fn(&Othello, i16) -> (usize, usize),
    black_iterations: i16,
    white_iterations: i16,
) {
    let mut black_wins = 0;
    let mut white_wins = 0;
    let mut draws = 0;

    let start_time = Instant::now();
    for i in 0..GAMES {
        print!("  game: {}/{}, elapsed time: {:.2}s\r", i + 1, GAMES, start_time.elapsed().as_secs_f32());
        std::io::stdout().flush().unwrap();

        let mut game = Othello::new();
        while [State::BlackTurn, State::WhiteTurn].contains(&game.state) {
            let position = if game.state == State::BlackTurn {
                black_ai(&game, black_iterations)
            } else {
                white_ai(&game, white_iterations)
            };
            game.make_move(position);
        }
        if game.state == State::BlackWon {
            black_wins += 1;
        } else if game.state == State::WhiteWon {
            white_wins += 1;
        } else {
            draws += 1;
        }
    }
    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("  elapsed time: {:.2}s                     ", elapsed_time);
    println!("    BLACK wins: {} {:.0} %", black_wins, (black_wins as f32 / GAMES as f32) * 100 as f32);
    println!("    WHITE wins: {} {:.0} %", white_wins, (white_wins as f32 / GAMES as f32) * 100 as f32);
    println!("         Draws: {} {:.0} %", draws, (draws as f32 / GAMES as f32) * 100 as f32);
}

fn random_move(game: &Othello, _dummy: i16) -> (usize, usize) {
    let valid_moves = game.get_valid_moves();
    valid_moves[rand::random::<usize>() % valid_moves.len()]
}
