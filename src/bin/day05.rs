use std::{
    cmp::{max, min},
    collections::VecDeque,
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

    // part 2
    let mut locs2: Vec<(u64, u64)> = Vec::new();
    for chunk in seeds.chunks(2) {
        if chunk.len() == 2 {
            locs2.push((chunk[0], chunk[1]));
        }
    }

    println!("locs2: {:?}", locs2);

    let mut currlocs: VecDeque<(u64, u64)> = locs2.into_iter().collect();
    for m in maps.iter() {
        // at each map level, we have a queue of currentlocs
        // go thru the queue and try to find a transformation that applies
        // if so, apply it and add that item to nextlocs
        // if none apply, add it to nextlocs wihtout transforming
        let mut nextlocs: VecDeque<(u64, u64)> = VecDeque::new();
        // println!("m: {:?}, currlocs: {:?}", m, currlocs);
        while let Some((loc_start, loc_len)) = currlocs.pop_front() {
            let mut transformed = false;
            for (src, dest, range) in m.iter() {
                let end = loc_start + loc_len;
                let src_end = *src + *range;
                // println!(
                //     // "loc_start: {}, end: {}, src: {}, src_end: {}",
                //     loc_start,
                //     end, src, src_end
                // );

                if loc_start < src_end && end > *src {
                    let overlap_start = max(loc_start, *src);
                    let overlap_end = min(end, src_end);
                    let overlap_len = overlap_end - overlap_start;
                    nextlocs.push_back((overlap_start - src + dest, overlap_len));
                    // println!("{:?}", (overlap_start - src + dest, overlap_len));
                    transformed = true;
                    if loc_start < *src {
                        currlocs.push_back((loc_start, *src - loc_start));
                        // println!("{:?}", (loc_start, *src - loc_start));
                    }
                    if end > src_end {
                        currlocs.push_back((src_end, end - src_end));
                        // println!("{:?}", (src_end, end - src_end));
                    }
                    break;
                }
            }
            if !transformed {
                nextlocs.push_back((loc_start, loc_len));
            }
        }
        currlocs = nextlocs;
    }

    println!("currlocs: {:?}", currlocs);
    let minloc = currlocs.iter().min().unwrap();
    println!("minloc: {}", minloc.0);

    // println!("Seeds: {:?}", seeds);
    // println!("Locations: {:?}", locs);
    // println!("min: {}", locs.iter().min().unwrap());
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
