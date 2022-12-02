use advent_of_code::helpers::vec_of_strings;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let mut score = 0;

    let point_values = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let left = parts[0];
        let right = parts[1];
        let points = result(left, right) + point_values[right];
        score += points;
    }

    Some(score)
}

// return 0 for loss, 3 for draw, 6 for win
fn result(left: &str, right: &str) -> u32 {
    if left == "A" {
        if right == "X" {
            3
        } else if right == "Y" {
            6
        } else {
            0
        }
    } else if left == "B" {
        if right == "X" {
            0
        } else if right == "Y" {
            3
        } else {
            6
        }
    } else if right == "X" {
        6
    } else if right == "Y" {
        0
    } else {
        3
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = vec_of_strings(input);
    let mut score = 0;

    let point_values = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6)
    ]);

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let left = parts[0];
        let right = parts[1];
        let points = result_two(left, right) + point_values[right];
        score += points;
    }

    Some(score)
}

fn result_two(left: &str, right: &str) -> u32 {
    if right == "X" {
        // lose case
        if left == "A" {
            3
        } else if left == "B" {
            1
        } else {
            2
        }
    } else if right == "Y" {
        // draw case
        if left == "A" {
            1
        } else if left == "B" {
            2
        } else {
            3
        }
    } else {
        // win case
        if left == "A" {
            2
        } else if left == "B" {
            3
        } else {
            1
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
