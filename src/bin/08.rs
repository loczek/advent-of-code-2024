use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
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

fn check_bounds(point: &Point, rows: i32, cols: i32) -> bool {
    if point.row < 0 || point.col < 0 || point.row >= rows || point.col >= cols {
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut count: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinode = HashSet::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '.' {
                count
                    .entry(grid[r][c])
                    .and_modify(|x| x.push(Point::new(r as i32, c as i32)))
                    .or_insert(vec![Point::new(r as i32, c as i32)]);
            }
        }
    }

    let rows = rows as i32;
    let cols = cols as i32;

    for nodes in count.values() {
        for node in nodes {
            for next_node in nodes {
                if node != next_node {
                    let diff = *next_node - *node;
                    let one = *next_node + diff;
                    if check_bounds(&one, rows, cols) {
                        antinode.insert(one);
                    }
                    let two = *node - diff;
                    if check_bounds(&two, rows, cols) {
                        antinode.insert(two);
                    }
                }
            }
        }
    }

    Some(antinode.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid.first().unwrap().len();
    let mut count: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinode = HashSet::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '.' {
                count
                    .entry(grid[r][c])
                    .and_modify(|x| x.push(Point::new(r as i32, c as i32)))
                    .or_insert(vec![Point::new(r as i32, c as i32)]);
            }
        }
    }

    let rows = rows as i32;
    let cols = cols as i32;

    for nodes in count.values() {
        for node in nodes {
            antinode.insert(*node);
            for next_node in nodes {
                if node != next_node {
                    let diff = *next_node - *node;
                    let mut one = *next_node + diff;
                    while check_bounds(&one, rows, cols) {
                        antinode.insert(one);
                        one = one + diff;
                    }
                    let mut two = *node - diff;
                    while check_bounds(&two, rows, cols) {
                        antinode.insert(two);
                        two = two - diff;
                    }
                }
            }
        }
    }

    Some(antinode.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
