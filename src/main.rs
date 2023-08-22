use std::env;

pub mod mcts;
pub mod minimax;
pub mod othello;
mod test;
//mod ui;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    test::run_tests();
    //ui::start_game();
}
