use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    let now = std::time::Instant::now();
    let file = File::open("inputs/day05.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut currmap: Vec<(u64, u64, u64)> = Vec::new();

    // part 1
    for line in lines.iter() {
        if line.contains("seeds") {
            seeds = line
                .split(':')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
        } else if line.contains("map") {
        } else if line.is_empty() {
            maps.push(currmap);
            currmap = Vec::new();
        } else if !line.is_empty() {
            fillmap(&mut currmap, line);
        }
    }
    maps.push(currmap);
    println!("num maps: {}", maps.len());

    let locs: Vec<u64> = seeds
        .iter()
        .map(|seed| {
            let mut loc = *seed;
            for map in maps.iter() {
                for (src, dest, range) in map.iter() {
                    if loc >= *src && loc < *src + range {
                        loc = (loc - src) + dest;
                        break;
                    }
                    // println!("loc: {}", loc);
                }
            }
            loc
        })
        .collect();

    println!("Seeds: {:?}", seeds);
    println!("Locations: {:?}", locs);
    println!("min: {}", locs.iter().min().unwrap());
    println!("Time: {}ms", now.elapsed().as_millis());
}

// generate several maps, apply each map.
// INCORRECT BUZZER
// Each map should actually be an array of tuples. (src, dest, range).
// Iterate over each array, if a number is in range, apply the transformation

fn fillmap(map: &mut Vec<(u64, u64, u64)>, line: &str) {
    let [dest, src, range] = line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    map.push((src, dest, range));
}
