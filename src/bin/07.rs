advent_of_code::solution!(7);

fn concat_digits(one: u64, two: u64) -> u64 {
    one as u64 * 10u64.pow(two.ilog10() + 1) + two
}

fn search_num_two(i: usize, curr: u64, target: u64, nums: &Vec<u64>) -> bool {
    if i == nums.len() {
        return curr == target;
    }

    let ans = false
        || search_num_two(i + 1, curr + nums[i], target, nums)
        || search_num_two(i + 1, curr * nums[i], target, nums);

    return ans;
}

fn search_num_three(i: usize, curr: u64, target: u64, nums: &Vec<u64>) -> bool {
    if i == nums.len() {
        return curr == target;
    }

    let ans = false
        || search_num_three(i + 1, curr + nums[i], target, nums)
        || search_num_three(i + 1, curr * nums[i], target, nums)
        || search_num_three(i + 1, concat_digits(curr, nums[i]), target, nums);

    return ans;
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let (total_str, nums_str) = line.split_once(": ").unwrap();
        let total = total_str.parse::<u64>().unwrap();
        let nums: Vec<u64> = nums_str
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        if search_num_two(0, 0, total, &nums) {
            ans += total;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let (total_str, nums_str) = line.split_once(": ").unwrap();
        let total = total_str.parse::<u64>().unwrap();
        let nums: Vec<u64> = nums_str
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        if search_num_three(0, 0, total, &nums) {
            ans += total;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
