use std::collections::HashMap;

advent_of_code::solution!(11);

fn count_digits(num: u64) -> u64 {
    num.ilog10() as u64 + (1 as u64)
}

fn split_digits(num: u64) -> (u64, u64) {
    let digits = num.ilog10() + 1;
    let mut i = digits;
    let mut first = 0;
    while i > digits / 2 {
        first *= 10;
        first += num / 10u64.pow(i - 1) % 10;
        i -= 1;
    }
    let mut second = 0;
    while i > 0 {
        second *= 10;
        second += num / 10u64.pow(i - 1) % 10;
        i -= 1;
    }
    (first, second)
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut freq: HashMap<u64, u64> = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut to_add = vec![];

    for _ in 0..25 {
        freq.iter_mut().for_each(|(key, count)| {
            if *key == 0 {
                to_add.push((1, count.clone()));
            } else if count_digits(*key) % 2 == 0 {
                let (one, two) = split_digits(*key);
                to_add.push((one, count.clone()));
                to_add.push((two, count.clone()));
            } else {
                to_add.push((*key * 2024, count.clone()));
            }

            *count = 0;
        });
        to_add.iter().for_each(|(key, count)| {
            *freq.entry(*key).or_insert(0) += *count;
        });
        to_add.clear();
        freq.retain(|_, count| *count != 0);
    }

    Some(freq.values().sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut freq: HashMap<u64, u64> = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut to_add = vec![];

    for _ in 0..75 {
        freq.iter_mut().for_each(|(key, count)| {
            if *key == 0 {
                to_add.push((1, count.clone()));
            } else if count_digits(*key) % 2 == 0 {
                let (one, two) = split_digits(*key);
                to_add.push((one, count.clone()));
                to_add.push((two, count.clone()));
            } else {
                to_add.push((*key * 2024, count.clone()));
            }

            *count = 0;
        });
        to_add.iter().for_each(|(key, count)| {
            *freq.entry(*key).or_insert(0) += *count;
        });
        to_add.clear();
        freq.retain(|_, count| *count != 0);
    }

    Some(freq.values().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
