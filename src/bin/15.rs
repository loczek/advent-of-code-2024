use advent_of_code::{Point, DIRS_POINT};

advent_of_code::solution!(15);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

fn can_move(grid: &mut Vec<Vec<char>>, pos: Point, dir: Point) -> bool {
    if grid[pos.row as usize][pos.col as usize] == '.' {
        return true;
    }

    if grid[pos.row as usize][pos.col as usize] == '#' {
        return false;
    }

    let next_point = pos + dir;
    let is_next_free = can_move(grid, next_point, dir);

    if is_next_free {
        let temp = grid[pos.row as usize][pos.col as usize];
        grid[next_point.row as usize][next_point.col as usize] = temp;
        grid[pos.row as usize][pos.col as usize] = '.';
    }

    is_next_free
}

fn can_move_vertically(grid: &mut Vec<Vec<char>>, pos: Point, dir: Point) -> bool {
    if grid[pos.row as usize][pos.col as usize] == '.' {
        return true;
    }

    if grid[pos.row as usize][pos.col as usize] == '#' {
        return false;
    }

    let left_pos = if grid[pos.row as usize][pos.col as usize] == '[' {
        pos
    } else {
        pos + Point::new(0, -1)
    };

    let right_pos = if grid[pos.row as usize][pos.col as usize] == ']' {
        pos
    } else {
        pos + Point::new(0, 1)
    };

    let is_left_free = can_move_vertically(grid, left_pos + dir, dir);
    let is_right_free = can_move_vertically(grid, right_pos + dir, dir);

    is_left_free && is_right_free
}

fn move_vertically(grid: &mut Vec<Vec<char>>, pos: Point, dir: Point) {
    if grid[pos.row as usize][pos.col as usize] == '.' {
        return;
    }

    if grid[pos.row as usize][pos.col as usize] == '#' {
        return;
    }

    let left_pos = if grid[pos.row as usize][pos.col as usize] == '[' {
        pos
    } else {
        pos + Point::new(0, -1)
    };

    let right_pos = if grid[pos.row as usize][pos.col as usize] == ']' {
        pos
    } else {
        pos + Point::new(0, 1)
    };

    let left_pos_next = left_pos + dir;
    let right_pos_next = right_pos + dir;

    move_vertically(grid, left_pos_next, dir);
    move_vertically(grid, right_pos_next, dir);

    let temp_left = grid[left_pos.row as usize][left_pos.col as usize];
    grid[left_pos_next.row as usize][left_pos_next.col as usize] = temp_left;
    grid[left_pos.row as usize][left_pos.col as usize] = '.';

    let temp_right = grid[right_pos.row as usize][right_pos.col as usize];
    grid[right_pos_next.row as usize][right_pos_next.col as usize] = temp_right;
    grid[right_pos.row as usize][right_pos.col as usize] = '.';
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .take_while(|x| x.starts_with("#"))
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let dirs = input
        .lines()
        .skip_while(|x| x.starts_with("#") || x.is_empty())
        .flat_map(|x| x.chars().map(|y| Direction::try_from(y).unwrap()))
        .collect::<Vec<_>>();

    let mut curr = Point::new(0, 0);

    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                curr = Point::new(r as i32, c as i32);
                break;
            }
        }
    }

    for direction in dirs {
        let dx = match direction {
            Direction::Up => DIRS_POINT[0],
            Direction::Right => DIRS_POINT[1],
            Direction::Down => DIRS_POINT[2],
            Direction::Left => DIRS_POINT[3],
        };

        if can_move(&mut grid, curr, dx) {
            curr = curr + dx;
        }
    }

    let mut ans: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'O' {
                ans += 100 * r as u32 + c as u32;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .take_while(|x| x.starts_with("#"))
        .map(|x| {
            x.chars()
                .flat_map(|x| match x {
                    '#' => "##".chars(),
                    'O' => "[]".chars(),
                    '.' => "..".chars(),
                    '@' => "@.".chars(),
                    _ => panic!("xd"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let dirs = input
        .lines()
        .skip_while(|x| x.starts_with("#") || x.is_empty())
        .flat_map(|x| x.chars().map(|y| Direction::try_from(y).unwrap()))
        .collect::<Vec<_>>();

    let mut curr = Point::new(0, 0);

    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                curr = Point::new(r as i32, c as i32);
                break;
            }
        }
    }

    for direction in dirs {
        let dx = match direction {
            Direction::Up => DIRS_POINT[0],
            Direction::Right => DIRS_POINT[1],
            Direction::Down => DIRS_POINT[2],
            Direction::Left => DIRS_POINT[3],
        };

        match direction {
            Direction::Up | Direction::Down => {
                if can_move_vertically(&mut grid, curr + dx, dx) {
                    move_vertically(&mut grid, curr + dx, dx);
                    let next_point = curr + dx;
                    let temp = grid[curr.row as usize][curr.col as usize];
                    grid[next_point.row as usize][next_point.col as usize] = temp;
                    grid[curr.row as usize][curr.col as usize] = '.';
                    curr = curr + dx;
                }
            }
            Direction::Left | Direction::Right => {
                if can_move(&mut grid, curr, dx) {
                    curr = curr + dx;
                }
            }
        }
    }

    let mut ans: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '[' {
                ans += 100 * r as u32 + c as u32;
            }
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
