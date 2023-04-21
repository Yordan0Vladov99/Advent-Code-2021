use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn is_valid(index: i32, size: i32) -> bool {
    return index >= 0 && index < size;
}

fn is_valid_pos(pos: (i32, i32), size: (i32, i32)) -> bool {
    return is_valid(pos.0, size.0) && is_valid(pos.1, size.1);
}

fn perform_step(
    pos: (i32, i32),
    grid: &mut Vec<Vec<u32>>,
    has_flashed: &mut [[bool; 10]; 10],
    flashes: &mut i32,
) {
    if !is_valid_pos(pos, (10, 10)) || has_flashed[pos.0 as usize][pos.1 as usize] {
        return;
    }

    let root = &mut grid[pos.0 as usize][pos.1 as usize];
    *root += 1;

    if *root > 9 {
        *root = 0;
        *flashes += 1;
        has_flashed[pos.0 as usize][pos.1 as usize] = true;

        for x in -1..=1 {
            for y in -1..=1 {
                perform_step((pos.0 + x, pos.1 + y), grid, has_flashed, flashes)
            }
        }
    }
}

fn perform_step_without_counting_flashes(
    pos: (i32, i32),
    grid: &mut Vec<Vec<u32>>,
    has_flashed: &mut [[bool; 10]; 10],
) {
    if !is_valid_pos(pos, (10, 10)) || has_flashed[pos.0 as usize][pos.1 as usize] {
        return;
    }

    let root = &mut grid[pos.0 as usize][pos.1 as usize];
    *root += 1;

    if *root > 9 {
        *root = 0;
        has_flashed[pos.0 as usize][pos.1 as usize] = true;

        for x in -1..=1 {
            for y in -1..=1 {
                perform_step_without_counting_flashes((pos.0 + x, pos.1 + y), grid, has_flashed)
            }
        }
    }
}

pub fn part_0() {
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<&str>> = Vec::new();
    for line in input.iter() {
        grid.push(line.split("").filter(|x| x.len() > 0).collect());
    }
    for line in grid.iter() {
        println!("{}", line.join(", "));
    }
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.iter() {
        grid.push(
            line.split("")
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    let mut flashes = 0;
    for _ in 0..100 {
        let mut has_flashed = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                perform_step((x, y), &mut grid, &mut has_flashed, &mut flashes);
            }
        }
    }

    println!("{flashes}")
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in input.iter() {
        grid.push(
            line.split("")
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect(),
        );
    }

    let mut iters = 0;
    loop {
        iters += 1;
        let mut has_flashed = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                perform_step_without_counting_flashes((x, y), &mut grid, &mut has_flashed);
            }
        }

        let mut all_flashed = true;

        has_flashed
            .iter()
            .for_each(|f| f.iter().for_each(|x| all_flashed &= *x));

        if all_flashed {
            println!("{}", iters);
            return;
        }
    }
}
