use std::collections::HashSet;

advent_of_code::solution!(10);

static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs(
    grid: &Vec<Vec<u32>>,
    visited: &mut Option<HashSet<(i32, i32)>>,
    rows: i32,
    cols: i32,
    row: i32,
    col: i32,
) -> u32 {
    if grid[row as usize][col as usize] == 9 {
        if let Some(visited) = visited {
            if !visited.contains(&(row, col)) {
                visited.insert((row, col));
                return 1;
            } else {
                return 0;
            }
        }

        return 1;
    }

    let mut ans = 0;

    for (dr, dc) in &DIRS {
        let r = row + dr;
        let c = col + dc;
        if r < 0 || r >= rows || c < 0 || c >= cols {
            continue;
        }
        if grid[row as usize][col as usize] + 1 == grid[r as usize][c as usize] {
            ans += dfs(grid, visited, rows, cols, r, c)
        }
    }

    return ans;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut ans = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                let visited: HashSet<(i32, i32)> = HashSet::new();
                ans += dfs(
                    &grid,
                    &mut Some(visited),
                    rows as i32,
                    cols as i32,
                    r as i32,
                    c as i32,
                );
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    let mut ans = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                ans += dfs(
                    &grid,
                    &mut None,
                    rows as i32,
                    cols as i32,
                    r as i32,
                    c as i32,
                );
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
