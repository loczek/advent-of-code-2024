use std::slice::Windows;

advent_of_code::solution!(2);

fn is_increasing(iter: Windows<'_, i32>) -> bool {
    for temp in iter {
        let left = temp[0];
        let right = temp[1];
        if left >= right || left.abs_diff(right) > 3 {
            return false;
        }
    }

    true
}

fn is_decreasing(iter: Windows<'_, i32>) -> bool {
    for temp in iter {
        let left = temp[0];
        let right = temp[1];
        if left <= right || left.abs_diff(right) > 3 {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if is_decreasing(levels.windows(2)) || is_increasing(levels.windows(2)) {
            ans += 1
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let levels = line
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut temp = false;

        if is_decreasing(levels.windows(2)) || is_increasing(levels.windows(2)) {
            temp = true;
        }

        for i in 0..levels.len() {
            let xd = levels
                .iter()
                .take(i)
                .chain(levels.iter().skip(i + 1))
                .map(|x| *x)
                .collect::<Vec<_>>();
            if is_decreasing(xd.windows(2)) || is_increasing(xd.windows(2)) {
                temp = true;
            }
        }

        if temp {
            ans += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
