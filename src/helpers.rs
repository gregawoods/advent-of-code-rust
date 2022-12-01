/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn vec_of_strings(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.split('\n').map(|str| {
        str.trim()
    }).filter(|str| {
        str != &""
    }).collect();

    lines
}

pub fn vec_of_numbers(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);

    return strings.iter().map(|str| {
        str.parse().unwrap()
    }).collect();
}
