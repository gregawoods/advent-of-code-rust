use advent_of_code::helpers::vec_of_strings_no_trim;
use array_tool::vec::Shift;
use itertools::rev;
use itertools::Itertools;
use regex::Regex;

struct Move {
    number: u32,
    from: usize,
    to: usize,
}

fn extract(input: &str) -> (Vec<Vec<String>>, Vec<Move>) {
    let mut stacks: Vec<Vec<String>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let regex = Regex::new(r"\d+").unwrap();

    for line in vec_of_strings_no_trim(input) {
        if line.contains('[') {
            for (index, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if stacks.len() <= index {
                    stacks.push(vec![]);
                }
                if chunk[1] != ' ' {
                    stacks[index].push(chunk[1].to_string());
                }
            }
        } else if line.starts_with("move") {
            let numbers: Vec<u32> = regex
                .find_iter(line)
                .filter_map(|m| m.as_str().parse().ok())
                .collect();

            moves.push(Move {
                number: numbers[0],
                from: usize::try_from(numbers[1] - 1).unwrap(),
                to: usize::try_from(numbers[2] - 1).unwrap(),
            });
        }
    }

    (stacks, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = extract(input);

    for m in moves {
        for _ in 0..m.number {
            let value = stacks[m.from].shift().unwrap();
            stacks[m.to].unshift(value);
        }
    }

    Some(stacks.iter().map(|stack| stack.first().unwrap()).join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = extract(input);

    for m in moves {
        let mut temp: Vec<String> = vec![];

        for _ in 0..m.number {
            let value = stacks[m.from].shift().unwrap();
            temp.push(value);
        }
        for value in rev(temp) {
            stacks[m.to].unshift(value);
        }
    }

    Some(stacks.iter().map(|stack| stack.first().unwrap()).join(""))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
