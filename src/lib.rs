use std::fmt::Display;

pub mod template;

pub static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub static DIRS_POINT: [Point; 4] = [
    Point { row: -1, col: 0 },
    Point { row: 0, col: 1 },
    Point { row: 1, col: 0 },
    Point { row: 0, col: -1 },
];

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub row: i32, // x
    pub col: i32, // y
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn in_bounds(&self, rows: i32, cols: i32) -> bool {
        if self.row < 0 || self.row >= rows || self.col < 0 || self.col >= cols {
            return false;
        }

        true
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}
