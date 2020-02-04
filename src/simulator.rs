pub mod plain_simulator;
pub mod save_bridge_simulator;

use crate::board::{Board};

pub trait Simulator {
    fn simulate(board: &Board) -> i32;
}