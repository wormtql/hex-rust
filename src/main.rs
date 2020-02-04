pub mod board;
pub mod solver;
pub mod common;
pub mod simulator;

use board::{Board, Color};
use simulator::*;
use simulator::plain_simulator::*;

fn main() {
    let sim = PlainSimulator::new();
    let board = Board::new(11);

    println!("{}", sim.simulate_many(&board, Color::Red, 200000));
}
