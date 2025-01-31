

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
const NUMS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn run() {
    let now = std::time::Instant::now();
    let file = File::open("inputs/day01.txt").unwrap();
    let reader = BufReader::new(file);  

    let sum: u32 = reader.lines().map(|line|{
        let line = line.unwrap();
        let first_digit = find_digit(&line);
        let second_digit = find_digit_reverse(&line);
        10*first_digit + second_digit
    }).sum();

    
    println!("Sum: {}", sum);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn find_digit(line: &str) -> u32 {
    for (i, c) in line.char_indices() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
        for (j, &num) in NUMS.iter().enumerate() {
            if line.get(i..i+num.len()) == Some(num) {
                return 1 + j as u32;
            }
        }
    }
    panic!("Could not find digit");
}

fn find_digit_reverse(line: &str) -> u32 {
    for (i, c) in line.char_indices().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
        for (j, &num) in NUMS.iter().enumerate() {
            let start = i.saturating_sub(num.len() - 1);
            if line.get(start..=i) == Some(num) {
                return 1 + j as u32;
            }
        }
    }
    panic!("Could not find digit");
}