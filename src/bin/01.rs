use advent_of_code::helpers::{vec_of_strings};
use itertools::max;

fn calculate(input: &str) -> Vec<u32> {
    let lines = vec_of_strings(input);
    let mut current = 0;
    let mut sums: Vec<u32> = vec![];

    for line in lines {
        if line.is_empty() {
            sums.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    sums.push(current);
    sums
}

pub fn part_one(input: &str) -> Option<u32> {
    max(calculate(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sums = calculate(input);
    sums.sort();
    
    let mut total = 0;

    for _i in 0..3 {
        let n = sums.pop().unwrap();
        total += n;
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
