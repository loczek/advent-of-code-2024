use std::collections::{HashSet, VecDeque};

use advent_of_code::{Point, DIRS_POINT};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let n = if cfg!(test) { 7 } else { 71 };

    let lines = input.lines();

    let mut points = vec![];

    for line in lines {
        let (left, right) = line.split_once(',').unwrap();
        let x = left.parse::<i32>().unwrap();
        let y = right.parse::<i32>().unwrap();

        points.push(Point::new(y, x));
    }

    let start = Point::new(0, 0);
    let end = Point::new(n - 1, n - 1);

    let mut fallen: HashSet<Point> = HashSet::new();
    let mut i = 0;

    while i < if cfg!(test) { 12 } else { 1024 } {
        fallen.insert(points[i]);
        i += 1;
    }

    let mut q: VecDeque<Point> = VecDeque::from([start]);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut ans = 0;

    'outer: while !q.is_empty() {
        for _ in 0..q.len() {
            let point = q.pop_front().unwrap();

            if point == end {
                break 'outer;
            }

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;

                if next_point.in_bounds(n, n)
                    && !visited.contains(&next_point)
                    && !fallen.contains(&next_point)
                {
                    visited.insert(next_point);
                    q.push_back(next_point);
                }
            }
        }

        ans += 1;
    }

    Some(ans)
}

fn can_reach_end(
    rows: i32,
    cols: i32,
    end: Point,
    q: &mut VecDeque<Point>,
    visited: &mut HashSet<Point>,
    fallen: &HashSet<Point>,
) -> bool {
    while !q.is_empty() {
        for _ in 0..q.len() {
            let point = q.pop_front().unwrap();

            if point == end {
                return true;
            }

            for point_dx in DIRS_POINT {
                let next_point = point + point_dx;

                if next_point.in_bounds(rows, cols)
                    && !visited.contains(&next_point)
                    && !fallen.contains(&next_point)
                {
                    visited.insert(next_point);
                    q.push_back(next_point);
                }
            }
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<Point> {
    let n = if cfg!(test) { 7 } else { 71 };

    let lines = input.lines();

    let mut points = vec![];

    for line in lines {
        let (left, right) = line.split_once(',').unwrap();
        let x = left.parse::<i32>().unwrap();
        let y = right.parse::<i32>().unwrap();

        points.push(Point::new(y, x));
    }

    let start = Point::new(0, 0);
    let end = Point::new(n - 1, n - 1);

    let q: VecDeque<Point> = VecDeque::from([start]);
    let visited: HashSet<Point> = HashSet::new();
    let mut fallen: HashSet<Point> = HashSet::new();
    let mut i = 0;

    while !q.is_empty() {
        if !can_reach_end(
            n,
            n,
            end,
            &mut q.clone(),
            &mut visited.clone(),
            &fallen.clone(),
        ) {
            return Some(points[i - 1]);
        }

        if i < points.len() {
            fallen.insert(points[i]);
            i += 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Point::new(1, 6)));
    }
}
