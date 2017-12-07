use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

pub fn solve() -> usize {
    let f = BufReader::new(File::open("resources/day_four.txt").unwrap());
    let valid :Vec<String> = f.lines()
        .map(|line| line.unwrap())
        .filter(|line| is_valid(line))
        .collect();

    return valid.len()
}

fn is_valid(input: &String) -> bool {
    let mut seen = HashSet::new();
    for word in input.split_whitespace() {
        if seen.contains(word) {
            return false
        } else {
            seen.insert(word);
        }
    }

    return true
}