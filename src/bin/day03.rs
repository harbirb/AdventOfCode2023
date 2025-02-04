use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

pub fn main() {
    let now = std::time::Instant::now();
    let file = File::open("inputs/day03.txt").unwrap();
    let reader = BufReader::new(file);
    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.unwrap().chars().collect()).collect();

    let sum = calculate_sum(&grid);
    let gsum = calculate_gear_sum(&grid);

    println!("Sum: {}", sum);
    println!("GSum: {}", gsum);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn calculate_sum(grid: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !grid[y][x].is_digit(10) && grid[y][x] != '.' {
                sum += sum_adjacent_numbers(grid, x, y);
            }
        }
    }
    sum
}

fn sum_adjacent_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    get_adjacent_numbers(grid, x, y).iter().sum()
}

fn calculate_gear_sum(grid: &Vec<Vec<char>>) -> u32 {
    let mut gsum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '*' {
                let nums = get_adjacent_numbers(grid, x, y);
                if nums.len() == 2 {
                    gsum += nums[0] * nums[1];
                }
            }
        }
    }
    gsum
}

fn get_adjacent_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    let mut nums = Vec::new();
    let dirs: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut visited = HashSet::new();

    for dir in dirs {
        let y2 = y as i32 + dir.0;
        let x2 = x as i32 + dir.1;

        if y2 >= 0 && y2 < grid.len() as i32 && x2 >= 0 && x2 < grid[0].len() as i32 {
            let y2 = y2 as usize;
            let x2 = x2 as usize;

            if grid[y2][x2].is_digit(10) {
                let mut x_start = x2 as i32;
                while x_start - 1 >= 0 && grid[y2][x_start as usize - 1].is_digit(10) {
                    x_start -= 1;
                }

                if !visited.contains(&(x_start, y2 as i32)) {
                    let num = extract_number(grid, x_start as usize, y2);
                    nums.push(num);
                    let mut x_current = x_start;
                    while x_current < grid[0].len() as i32 && grid[y2][x_current as usize].is_digit(10) {
                        visited.insert((x_current, y2 as i32));
                        x_current += 1;
                    }
                }
            }
        }
    }
    nums
}

fn extract_number(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut xleft = x as i32;
    let mut xright = x as i32;

    while xleft - 1 >= 0 && grid[y][xleft as usize - 1].is_digit(10) {
        xleft -= 1;
    }
    while xright + 1 < grid[0].len() as i32 && grid[y][xright as usize + 1].is_digit(10) {
        xright += 1;
    }

    let mut num = 0;
    for x in xleft..=xright {
        num = num * 10 + grid[y][x as usize].to_digit(10).unwrap();
    }
    num
}