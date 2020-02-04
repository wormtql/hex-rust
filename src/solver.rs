pub mod mcts_solver;

use crate::board::{Board};
use crate::common::{Move};

pub trait Solver {
    fn solve(board: &Board) -> Move;
}