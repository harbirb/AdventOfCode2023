use std::{cmp::max, collections::HashMap, fs::File, io::{BufRead, BufReader}};

pub fn main() {
    let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();
    let now = std::time::Instant::now();
    let file = File::open("inputs/day02.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // part 1
    let sum: u32 = lines.iter().map(|line| {
        let game: Vec<&str> = line.split(':').collect();
        let id = game.get(0).unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
        if game.last().unwrap().split(';').map(|set| {
            set.split(',').map(|cube| {
                let num = cube.split_whitespace().next().unwrap().parse::<u32>().unwrap();
                let color = cube.split_whitespace().last().unwrap();
                limits.get(color).unwrap() >= &num
            }).all(|x| x)
        }).all(|x| x) {
            return id;
        }
        0
    }).sum();   
    println!("Sum: {}", sum);
    println!("Time: {}ms", now.elapsed().as_millis());

    // part 2
    let psum: u32 = lines.iter().map(|line| {
        let game: Vec<&str> = line.split(':').collect();
        // let id = game.get(0).unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
        let (r, g, b) = game.last().unwrap().split(';').map(|set| {
            set.split(',').map(|cube| {
                let num = cube.split_whitespace().next().unwrap().parse::<u32>().unwrap();
                let color = cube.split_whitespace().last().unwrap();
                return (num, color);
            }).fold((0, 0, 0), |(max_r, max_g, max_b), (num, color)| {
                match color {
                    "red" => (max_r.max(num), max_g, max_b),
                    "green" => (max_r, max_g.max(num), max_b),
                    "blue" => (max_r, max_g, max_b.max(num)),
                    _ => panic!("Invalid color"),
                }
            })
        }).fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
            (max(max_r, r), max(max_g, g), max(max_b, b))
        });
        return r*g*b;
    }).sum();
    println!("pSum: {}", psum);
}
