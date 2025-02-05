use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    let now = std::time::Instant::now();
    let file = File::open("inputs/day04.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().split(':').nth(1).unwrap().to_string())
        .collect();

    let points: u32 = lines
        .iter()
        .map(|line| {
            let (winners, matches) = line.split_once('|').unwrap();
            let winners = parse_numbers(winners);
            let matches = parse_numbers(matches)
                .iter()
                .filter(|x| winners.contains(x))
                .count() as u32;
            if matches == 0 {
                0
            } else {
                2u32.pow(matches - 1)
            }
        })
        .sum();

    println!("Points: {}", points);

    // part 2
    let mut cards: HashMap<u32, u32> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let cardno: u32 = i as u32 + 1;
        cards.entry(cardno).or_insert(1);
        let (winners, matches) = line.split_once('|').unwrap();
        let winners = parse_numbers(winners);
        let matches = parse_numbers(matches)
            .iter()
            .filter(|x| winners.contains(x))
            .count() as u32;
        let copies = cards[&cardno];
        for j in 1..=matches {
            *cards.entry(cardno + j).or_insert(1) += copies;
        }
    }
    let total = cards.values().sum::<u32>();
    println!("Total: {}", total);

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn parse_numbers(s: &str) -> Vec<u32> {
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
