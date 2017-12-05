use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve() -> String {
    let f = BufReader::new(File::open("resources/day_two.txt").unwrap());
    let checksum :u32 = f.lines()
        .map(|line| line.unwrap())
        .map(|line| line_checksum(line))
        .sum();

    return checksum.to_string()
}

fn line_checksum(line: String) -> u32 {
    let numbers :Vec<u32> = line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();

    return max - min
}

pub fn solve_two() -> String {
    let f = BufReader::new(File::open("resources/day_two.txt").unwrap());
    let checksum :u32 = f.lines()
        .map(|line| line.unwrap())
        .map(|line| line_checksum_two(line))
        .sum();

    return checksum.to_string()
}

fn line_checksum_two(line: String) -> u32 {
    let numbers :Vec<u32> = line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    for number in numbers.iter() {
        for digit in numbers.iter() {
            if number != digit {
                if number % digit == 0 {
                    println!("{} / {} = {}", number, digit, number / digit);
                    return number / digit
                }
            }
        }
    }

    panic!("oh no")
}