use advent_of_code::helpers::vec_of_strings;
use regex::Regex;
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let regex = Regex::new(r"[,-]").unwrap();
    let mut count = 0;

    for line in lines {
        let numbers: Vec<u32> = regex.split(line).map(|str| str.parse().unwrap()).collect();

        if numbers[0] <= numbers[2] && numbers[1] >= numbers[3]
            || numbers[0] >= numbers[2] && numbers[1] <= numbers[3]
        {
            count += 1;
        }
    }

    Some(count)
}

fn range_intersects<T: PartialOrd>(a: RangeInclusive<T>, b: RangeInclusive<T>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let regex = Regex::new(r"[,-]").unwrap();
    let mut count = 0;

    for line in lines {
        let numbers: Vec<u32> = regex.split(line).map(|str| str.parse().unwrap()).collect();

        if range_intersects(numbers[0]..=numbers[1], numbers[2]..=numbers[3]) {
            count += 1;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
