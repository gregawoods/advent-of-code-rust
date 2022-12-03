use advent_of_code::helpers::{vec_of_strings, index_of_substring};
use array_tool::vec::Intersect;

static ALPHABET:&str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let mut sum:u32 = 0;

    for line in lines {
        let len = line.chars().count() / 2;
        let left:Vec<char> = line[..len].chars().collect();
        let right:Vec<char> = line[len..].chars().collect();
        let result = left.intersect(right).first().unwrap().to_string();

        sum += 1 + index_of_substring(ALPHABET, &result)
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let mut sum:u32 = 0;

    for chunk in lines.chunks(3) {
        let str_a: Vec<char> = chunk[0].chars().collect();
        let str_b: Vec<char> = chunk[1].chars().collect();
        let str_c: Vec<char> = chunk[2].chars().collect();
        let result = str_a.intersect(str_b)
                                  .intersect(str_c)
                                  .first().unwrap().to_string();

        sum += 1 + index_of_substring(ALPHABET, &result)
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
