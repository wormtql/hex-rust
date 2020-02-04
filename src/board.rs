use std::fmt;

const DIR: [(i32, i32); 6] = [
    (0, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, 1)
];

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Color {
    Red,
    Blue,
    Empty
}

impl Color {
    pub fn flip(&self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
            Color::Empty => Color::Empty
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "{}", "red"),
            Color::Blue => write!(f, "{}", "blue"),
            Color::Empty => write!(f, "{}", "empty"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    pub data: Vec<Vec<Color>>,
    pub size: i32
}

impl Board {
    pub fn new(size: i32) -> Board {
        Board {
            size,
            data: vec![vec![Color::Empty; size as usize]; size as usize]
        }
    }
}

impl Board {
    pub fn is_player_won(&self, player: Color) -> bool {
        if player == Color::Empty {
            return false;
        }

        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut vis: Vec<Vec<bool>> = vec![vec![false; self.size as usize]; self.size as usize];

        if player == Color::Red {
            for i in 0..self.size as usize {
                if self.data[0][i] == Color::Red {
                    stack.push((0, i as i32));
                    vis[0][i] = true;
                }
            }
        } else if player == Color::Blue {
            for i in 0..self.size as usize {
                if self.data[i][0] == Color::Blue {
                    stack.push((i as i32, 0));
                    vis[i][0] = true;
                }
            }
        }

        while !stack.is_empty() {
            let p = stack.pop().unwrap();
            for &(dx, dy) in DIR.iter() {
                let nx = p.0 + dx;
                let ny = p.1 + dy;
                if nx >= 0 && nx < self.size && ny >= 0 && ny < self.size {
                    if self.data[nx as usize][ny as usize] == player{
                        if player == Color::Red && nx == self.size - 1 {
                            return true;
                        } else if player == Color::Blue && ny == self.size - 1 {
                            return true;
                        }

                        if !vis[nx as usize][ny as usize] {
                            stack.push((nx, ny));
                            vis[nx as usize][ny as usize] = true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl Board {
    pub fn set_x_y(&mut self, x: i32, y: i32, color: Color) {
        self.data[x as usize][y as usize] = color;
    }

    pub fn set_pos(&mut self, position: i32, color: Color) {
        let x = position / self.size;
        let y = position % self.size;
        self.data[x as usize][y as usize] = color;
    }

    pub fn get_x_y(&self, x: i32, y: i32) -> Color {
        self.data[x as usize][y as usize]
    }

    pub fn get_pos(&self, position: i32) -> Color {
        let x = position / self.size;
        let y = position % self.size;
        self.data[x as usize][y as usize]
    }

    pub fn winner_definite(&self) -> Color {
        if self.is_player_won(Color::Red) {
            Color::Red
        } else {
            Color::Blue
        }
    }

    pub fn winner(&self) -> Color {
        if self.is_player_won(Color::Red) {
            Color::Red
        } else if self.is_player_won(Color::Blue) {
            Color::Blue
        } else {
            Color::Empty
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size as usize {
            for _ in 0..i {
                write!(f, "{}", " ")?;
            }
            for j in 0..self.size as usize {
                write!(f, "{} ", match self.data[i][j] {
                    Color::Blue => "o",
                    Color::Red => "x",
                    Color::Empty => "."
                })?;
            }
            write!(f, "{}", "\n")?;
        }
        fmt::Result::Ok(())
    }
}

#[test]
fn test_winner() {
    let board = Board {
        size: 3,
        data: vec![
            vec![Color::Red, Color::Empty, Color::Empty],
            vec![Color::Red, Color::Red, Color::Blue],
            vec![Color::Blue, Color::Red, Color::Empty],
        ]
    };

    assert_eq!(true, board.is_player_won(Color::Red));
    assert_eq!(false, board.is_player_won(Color::Empty));
    assert_eq!(false, board.is_player_won(Color::Blue));

    let board = Board {
        size: 3,
        data: vec![
            vec![Color::Red, Color::Empty, Color::Empty],
            vec![Color::Red, Color::Red, Color::Blue],
            vec![Color::Blue, Color::Blue, Color::Blue],
        ]
    };

    assert_eq!(true, board.is_player_won(Color::Blue));
    assert_eq!(false, board.is_player_won(Color::Red));
}

#[test]
fn test_color() {
    let c = Color::Red;
    let c = c.flip();
    assert_eq!(c, Color::Blue);

    let c = c.flip();
    assert_eq!(c, Color::Red);
}