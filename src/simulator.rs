pub mod plain_simulator;
pub mod save_bridge_simulator;

use crate::board::{Board, Color};

pub trait Simulator {
    fn simulate(&self, board: &Board, next_player: Color) -> Color;

    fn simulate_many(&self, board: &Board, next_player: Color, times: i32) -> i32 {
        let mut win_red = 0_i32;
        for _ in 0..times {
            if self.simulate(board, next_player) == Color::Red {
                win_red += 1;
            }
        }
        win_red
    }
}