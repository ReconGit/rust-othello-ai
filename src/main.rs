pub mod mcts;
pub mod minimax;
pub mod othello;
//pub mod benchmark;
mod ui;

fn main() {
    ui::start_game();
    //benchmark::run_tests();
}
