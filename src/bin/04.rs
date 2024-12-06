advent_of_code::solution!(4);

fn find_xmas(
    grid: &Vec<Vec<char>>,
    rows: i32,
    cols: i32,
    r: i32,
    c: i32,
    row_offset: i32,
    col_offset: i32,
    word: &str,
) -> bool {
    for i in 0..word.len() as i32 {
        if r + (row_offset * i) < 0
            || c + (col_offset * i) < 0
            || r + (row_offset * i) >= rows
            || c + (col_offset * i) >= cols
            || grid[(r + (row_offset * i)) as usize][(c + (col_offset * i)) as usize]
                != word.chars().nth(i as usize).unwrap()
        {
            return false;
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len() as i32;
    let cols = grid.first().unwrap().len() as i32;

    let target = "XMAS";
    let mut ans = 0;

    for r in 0..rows {
        for c in 0..cols {
            ans += vec![
                find_xmas(&grid, rows, cols, r, c, 1, 0, target),
                find_xmas(&grid, rows, cols, r, c, -1, 0, target),
                find_xmas(&grid, rows, cols, r, c, 0, 1, target),
                find_xmas(&grid, rows, cols, r, c, 0, -1, target),
                find_xmas(&grid, rows, cols, r, c, 1, 1, target),
                find_xmas(&grid, rows, cols, r, c, 1, -1, target),
                find_xmas(&grid, rows, cols, r, c, -1, 1, target),
                find_xmas(&grid, rows, cols, r, c, -1, -1, target),
            ]
            .iter()
            .map(|x| *x as u32)
            .sum::<u32>();
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len() as i32;
    let cols = grid.first().unwrap().len() as i32;

    let target = "MAS";
    let mut ans = 0;

    for r in 0..rows {
        for c in 0..cols {
            ans += vec![
                find_xmas(&grid, rows, cols, r - 1, c - 1, 1, 1, target)
                    && find_xmas(&grid, rows, cols, r - 1, c + 1, 1, -1, target),
                find_xmas(&grid, rows, cols, r - 1, c - 1, 1, 1, target)
                    && find_xmas(&grid, rows, cols, r + 1, c - 1, -1, 1, target),
                find_xmas(&grid, rows, cols, r + 1, c + 1, -1, -1, target)
                    && find_xmas(&grid, rows, cols, r - 1, c + 1, 1, -1, target),
                find_xmas(&grid, rows, cols, r + 1, c + 1, -1, -1, target)
                    && find_xmas(&grid, rows, cols, r + 1, c - 1, -1, 1, target),
            ]
            .iter()
            .map(|x| *x as u32)
            .sum::<u32>();
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
