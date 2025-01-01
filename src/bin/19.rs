use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

fn match_pattern(
    patterns: &HashSet<&str>,
    target: &String,
    target_start: usize,
    target_end: usize,
) -> bool {
    if target_start == target.len() {
        return true;
    }

    if target_end > target.len() {
        return false;
    }

    let mut ans = match_pattern(patterns, target, target_start, target_end + 1);

    if patterns.contains(&target[target_start..target_end]) {
        ans = ans || match_pattern(patterns, target, target_end, target_end + 1)
    }

    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let mut patterns = HashSet::new();

    for pattern in lines.next().unwrap().split(", ") {
        patterns.insert(pattern);
    }

    lines.next();

    let mut ans = 0;

    for design in lines {
        if match_pattern(&patterns, &design.to_string(), 0, 1) {
            ans += 1;
        }
    }

    Some(ans)
}

fn match_pattern_with_cache(
    patterns: &HashSet<&str>,
    cache: &mut HashMap<usize, u64>,
    target: &String,
    target_start: usize,
    target_end: usize,
) -> u64 {
    if let Some(count) = cache.get(&(target_start)) {
        return *count;
    }

    if target_start == target.len() {
        return 1;
    }

    if target_end > target.len() {
        return 0;
    }

    let mut ans = match_pattern_with_cache(patterns, cache, target, target_start, target_end + 1);

    if patterns.contains(&target[target_start..target_end]) {
        ans += match_pattern_with_cache(patterns, cache, target, target_end, target_end + 1);
    }

    cache.insert(target_start, ans);

    ans
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut patterns = HashSet::new();

    for pattern in lines.next().unwrap().split(", ") {
        patterns.insert(pattern);
    }

    lines.next();

    let mut ans = 0;

    for design in lines {
        let mut cache: HashMap<usize, u64> = HashMap::new();
        ans += match_pattern_with_cache(&patterns, &mut cache, &design.to_string(), 0, 1);
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
