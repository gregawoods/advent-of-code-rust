use array_tool::vec::{Shift, Uniq};

fn calculate(input: &str, limit: usize) -> Option<u32> {
    let mut list: Vec<char> = vec![];
    let mut result: u32 = 0;

    for (index, char) in input.chars().enumerate() {
        list.push(char);

        if list.len() > limit {
            list.shift();
        }

        if list.unique().len() == limit {
            result = u32::try_from(index).unwrap();
            break;
        }
    }

    Some(result + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    calculate(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    calculate(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
