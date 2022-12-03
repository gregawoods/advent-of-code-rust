/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn vec_of_strings(input: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = input.split('\n').map(|str| {
        str.trim()
    }).collect();

    if lines.last().unwrap() == &"" {
        lines.pop();
    }

    lines
}

pub fn vec_of_numbers(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);

    return strings.iter().map(|str| {
        str.parse().unwrap()
    }).collect();
}

pub fn index_of_substring(haystack: &str, needle: &str) -> u32 {
    let index = haystack.find(needle).unwrap();
    

    u32::try_from(index).unwrap()
}
