pub mod board;
pub mod solver;
pub mod common;
pub mod simulator;

use board::{Board, Color};

fn main() {
    let mut board = Board::new(5);
    board.set_x_y(2, 3, Color::Red);
    board.set_x_y(3, 2, Color::Blue);
    println!("{}", board);
}
