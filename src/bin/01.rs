use advent_of_code::helpers::vec_of_numbers;

pub fn part_one(input: &str) -> Option<u32> {
    let numbers = vec_of_numbers(input);
    Some(calculate(numbers))
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = vec_of_numbers(input);

    let mut last1 = 0;
    let mut last2 = 0;
    let mut sums: Vec<u32> = vec![];

    for n in numbers {
        if last1 != 0 && last2 != 0 {
            sums.push(last1 + last2 + n);
        }
        last1 = last2;
        last2 = n;
    }

    Some(calculate(sums))
}

fn calculate(numbers: Vec<u32>) -> u32 {
    let mut last: u32 = 0;
    let mut sum: u32 = 0;

    for n in numbers {
        if last != 0 && n > last {
            sum += 1
        }
        last = n
    }

    sum
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
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
