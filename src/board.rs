use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Color {
    Red,
    Blue,
    Empty
}

impl Color {
    pub fn flip(&mut self) {
        match &self {
            Red => *self = Color::Blue,
            Blue => *self = Color::Red,
            Empty => ()
        };
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

    pub fn flip_x_y(&mut self, x: i32, y: i32) {

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