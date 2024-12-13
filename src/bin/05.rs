use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

fn has_correct_order(nums: &Vec<u32>, before: &HashMap<u32, Vec<u32>>) -> bool {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if let Some(x) = before.get(&nums[i]) {
                if x.contains(&nums[j]) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut iter = lines.iter().peekable();
    let rules = iter
        .by_ref()
        .take_while(|x| **x != "")
        .map(|x| *x)
        .collect::<Vec<&str>>();
    let updates = iter.map(|x| *x).collect::<Vec<_>>();
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules {
        let (u, v) = rule
            .split_once('|')
            .map(|(u, v)| (u.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()))
            .unwrap();

        before.entry(v).and_modify(|x| x.push(u)).or_insert(vec![u]);
    }

    let mut ans = 0;

    for update in updates {
        let nums = update
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        if has_correct_order(&nums, &before) {
            let mut iter = nums.iter();
            ans += iter.nth(nums.len() / 2).unwrap();
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut iter = lines.iter().peekable();
    let rules = iter
        .by_ref()
        .take_while(|x| **x != "")
        .map(|x| *x)
        .collect::<Vec<&str>>();
    let updates = iter.map(|x| *x).collect::<Vec<_>>();
    let mut before: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules {
        let (u, v) = rule
            .split_once('|')
            .map(|(u, v)| (u.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()))
            .unwrap();

        before.entry(v).and_modify(|x| x.push(u)).or_insert(vec![u]);
    }

    let mut ans = 0;

    for update in updates {
        let mut nums = update
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        if !has_correct_order(&nums, &before) {
            nums.sort_by(|a, b| {
                if let Some(x) = before.get(a) {
                    if x.contains(b) {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            let mut iter = nums.iter();
            ans += iter.nth(nums.len() / 2).unwrap();
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
