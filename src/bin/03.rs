use std::{
    iter::{Peekable, Skip},
    str::Chars,
};

advent_of_code::solution!(3);

fn match_string(iter: &mut Peekable<Skip<Chars<'_>>>, pattern: &str) -> bool {
    for c in pattern.chars() {
        if iter.peek().is_some() && iter.next().unwrap() != c {
            return false;
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        for i in 0..line.len() {
            let mut iter = line.chars().skip(i).peekable();

            if line.chars().nth(i).unwrap() == 'm' {
                if !match_string(&mut iter, "mul") || !match_string(&mut iter, "(") {
                    continue;
                }

                let mut one = 0;

                while let Some(num) = iter
                    .next_if(|x| x.is_digit(10))
                    .map(|x| x.to_digit(10).unwrap())
                {
                    one *= 10;
                    one += num;
                }

                if one <= 1 && one >= 1000 {
                    continue;
                }

                if !match_string(&mut iter, ",") {
                    continue;
                }

                let mut two = 0;

                while let Some(num) = iter
                    .next_if(|x| x.is_digit(10))
                    .map(|x| x.to_digit(10).unwrap())
                {
                    two *= 10;
                    two += num;
                }

                if two <= 0 || two >= 1000 {
                    continue;
                }

                if !match_string(&mut iter, ")") {
                    continue;
                }

                ans += one * two;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut ans = 0;

    let mut is_enabled = true;

    for line in lines {
        for i in 0..line.len() {
            let mut iter = line.chars().skip(i).peekable();

            if line.chars().nth(i).unwrap() == 'm' && is_enabled {
                if !match_string(&mut iter, "mul") || !match_string(&mut iter, "(") {
                    continue;
                }

                let mut one = 0;

                while let Some(num) = iter
                    .next_if(|x| x.is_digit(10))
                    .map(|x| x.to_digit(10).unwrap())
                {
                    one *= 10;
                    one += num;
                }

                if one <= 1 && one >= 1000 {
                    continue;
                }

                if !match_string(&mut iter, ",") {
                    continue;
                }

                let mut two = 0;

                while let Some(num) = iter
                    .next_if(|x| x.is_digit(10))
                    .map(|x| x.to_digit(10).unwrap())
                {
                    two *= 10;
                    two += num;
                }

                if two <= 0 || two >= 1000 {
                    continue;
                }

                if !match_string(&mut iter, ")") {
                    continue;
                }

                ans += one * two;
            } else if line.chars().nth(i).unwrap() == 'd' {
                let mut do_iter = iter.clone();
                let mut do_not_iter = iter.clone();

                if match_string(&mut do_iter, "do()") {
                    is_enabled = true;
                } else if match_string(&mut do_not_iter, "don't()") {
                    is_enabled = false
                }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
