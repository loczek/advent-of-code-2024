use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let mut split = line.split_whitespace();

        if let (Some(l), Some(r)) = (split.next(), split.next()) {
            left.push(l.parse::<u32>().unwrap());
            right.push(r.parse::<u32>().unwrap());
        }
    }

    left.sort_unstable();
    right.sort_unstable();

    Some(zip(left.into_iter(), right.into_iter()).fold(0, |acc, (l, r)| acc + l.abs_diff(r)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut left = vec![];
    let mut count = HashMap::new();

    for line in lines {
        let mut split = line.split_whitespace();

        if let (Some(l), Some(r)) = (split.next(), split.next()) {
            left.push(l.parse::<u32>().unwrap());
            count
                .entry(r.parse::<u32>().unwrap())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    Some(left.into_iter().fold(0, |acc, curr| {
        acc + (curr * count.get(&curr).or(Some(&0)).unwrap())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
