use super::{Simulator};
use crate::board::{Board, Color};

struct PlainSimulator {

}

impl PlainSimulator {
    pub fn new() -> PlainSimulator {
        PlainSimulator {}
    }
}

impl Simulator for PlainSimulator {
    fn simulate(board: &Board) -> i32 {
        0
    }
}