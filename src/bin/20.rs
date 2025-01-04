use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{in_bounds, Point, DIRS_POINT};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut start = Point::new(0, 0);

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = Point::new(r as i32, c as i32);
                break;
            }
        }
    }

    let mut q = VecDeque::from([start]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut distance: HashMap<Point, i32> = HashMap::new();
    let mut i = 0;

    distance.insert(start, 0);

    while !q.is_empty() {
        for _ in 0..q.len() {
            let point = q.pop_front().unwrap();

            visited.insert(point);
            distance.insert(point, i);

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;
                if next_point.in_bounds(rows as i32, cols as i32)
                    && grid[next_point.row as usize][next_point.col as usize] != '#'
                    && !visited.contains(&next_point)
                {
                    q.push_back(next_point);
                }
            }

            i += 1
        }
    }

    let mut q = VecDeque::from([start]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut ans = 0;

    while !q.is_empty() {
        for _ in 0..q.len() {
            let point = q.pop_front().unwrap();

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;
                if in_bounds(next_point.row, rows as i32, next_point.col, cols as i32)
                    && grid[next_point.row as usize][next_point.col as usize] == '.'
                    && !visited.contains(&next_point)
                {
                    q.push_back(next_point);
                }
            }

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;
                if next_point.in_bounds(rows as i32, cols as i32)
                    && grid[next_point.row as usize][next_point.col as usize] == '#'
                    && !visited.contains(&next_point)
                {
                    let next_next_point = point + point_dx + point_dx;
                    if next_next_point.in_bounds(rows as i32, cols as i32)
                        && grid[next_next_point.row as usize][next_next_point.col as usize] != '#'
                        && !visited.contains(&next_next_point)
                    {
                        let saved = distance.get(&next_next_point).unwrap()
                            - distance.get(&point).unwrap()
                            - 2;
                        if saved >= if cfg!(test) { 2 } else { 100 } {
                            ans += 1;
                        }
                    }
                }
            }

            visited.insert(point);

            i += 1
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut start = Point::new(0, 0);

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = Point::new(r as i32, c as i32);
                break;
            }
        }
    }

    let mut q = VecDeque::from([start]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut distance: HashMap<Point, i32> = HashMap::new();
    let mut i = 0;

    distance.insert(start, 0);

    while !q.is_empty() {
        for _ in 0..q.len() {
            let point = q.pop_front().unwrap();

            visited.insert(point);
            distance.insert(point, i);

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;
                if next_point.in_bounds(rows as i32, cols as i32)
                    && grid[next_point.row as usize][next_point.col as usize] != '#'
                    && !visited.contains(&next_point)
                {
                    q.push_back(next_point);
                }
            }

            i += 1
        }
    }

    let ans = distance
        .par_iter()
        .map(|(skip_start, _)| {
            let mut q_inner = VecDeque::from([*skip_start]);
            let mut visited_inner = HashSet::new();
            let mut j = 0;
            let mut ans = 0;

            while !q_inner.is_empty() && j < 21 {
                for _ in 0..q_inner.len() {
                    let skip_end = q_inner.pop_front().unwrap();

                    if grid[skip_end.row as usize][skip_end.col as usize] != '#' {
                        let diff =
                            distance.get(&skip_end).unwrap() - distance.get(&skip_start).unwrap();
                        let skipped = j;
                        let saved = diff - skipped;

                        if saved >= if cfg!(test) { 50 } else { 100 } {
                            ans += 1;
                        }
                    }

                    for point_dx in DIRS_POINT {
                        let next_point = skip_end + point_dx;
                        if next_point.in_bounds(rows as i32, cols as i32)
                            && !visited_inner.contains(&next_point)
                        {
                            q_inner.push_back(next_point);
                            visited_inner.insert(next_point);
                        }
                    }
                }

                j += 1;
            }

            ans
        })
        .sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
