use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        grid.push(line.chars().collect());
    }
    let mut oxygen_grid = grid.clone();
    let mut scrubber_grid = grid.clone();

    let mut i = 0;
    while oxygen_grid.len() > 1 && i < grid[0].len() {
        let mut ones = 0;
        let mut zeros = 0;

        for row in 0..oxygen_grid.len() {
            match oxygen_grid[row][i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let most_common = if zeros > ones { '0' } else { '1' };

        oxygen_grid = oxygen_grid
            .into_iter()
            .filter(|x| x[i] == most_common)
            .collect();
        i += 1;
    }
    i = 0;
    while scrubber_grid.len() > 1 && i < grid[0].len() {
        let mut ones = 0;
        let mut zeros = 0;

        for row in 0..scrubber_grid.len() {
            match scrubber_grid[row][i] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        let least_common = if zeros > ones { '1' } else { '0' };

        scrubber_grid = scrubber_grid
            .into_iter()
            .filter(|x| x[i] == least_common)
            .collect();
        i += 1;
    }

    let oxygen_val: u64 =
        u64::from_str_radix(&oxygen_grid[0].iter().collect::<String>(), 2).unwrap();
    let scrubber_val: u64 =
        u64::from_str_radix(&scrubber_grid[0].iter().collect::<String>(), 2).unwrap();

    println!("{}", oxygen_val * scrubber_val);
}

pub fn part_1() {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    let input = get_input("input.txt".to_string());
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        grid.push(line.chars().collect());
    }

    for col in 0..grid[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => {}
            }
        }

        if ones > zeros {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_rate_val: u64 = u64::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate_val: u64 = u64::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("{}", gamma_rate_val * epsilon_rate_val);
}
