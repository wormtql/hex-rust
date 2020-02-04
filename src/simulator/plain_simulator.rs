use super::{Simulator};
use crate::board::{Board, Color};
use rand::prelude::*;

pub struct PlainSimulator {

}

impl PlainSimulator {
    pub fn new() -> PlainSimulator {
        PlainSimulator {}
    }
}

impl Simulator for PlainSimulator {
    fn simulate(&self, board: &Board, next_player: Color) -> Color {
        let mut rng = thread_rng();

        let mut board_clone = board.clone();
        let mut valid_move = Vec::new();
        for i in 0..board.size * board.size {
            valid_move.push(i);
        }
        let mut valid_move_count = board.size * board.size;

        let mut player = next_player;

        for _ in 0..board.size * board.size {
            let index = rng.gen_range(0, valid_move_count);
            let pos = valid_move[index as usize];
            
            if board_clone.get_pos(pos) == Color::Empty {
                board_clone.set_pos(pos, player);
                player = player.flip();

                valid_move.swap(index as usize, valid_move_count as usize - 1);
                valid_move_count -= 1;
            }
        }

        // println!("{}", board_clone);

        board_clone.winner_definite()
    }
}

