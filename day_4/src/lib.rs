use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut cards: Vec<(HashMap<u32, (usize, usize)>, Vec<Vec<bool>>)> = Vec::new();

    let draws: Vec<u32> = input[0].split(",").map(|x| x.parse().unwrap()).collect();

    for i in (2..input.len()).step_by(6) {
        let mut positions: HashMap<u32, (usize, usize)> = HashMap::new();

        for j in 0..5 {
            let row: Vec<u32> = input[i + j]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for k in 0..row.len() {
                positions.insert(row[k], (j, k));
            }
        }

        cards.push((positions, vec![vec![false; 5]; 5]));
    }

    for draw in draws.iter() {
        for i in 0..cards.len() {
            let (pos, marked) = &mut cards[i];

            if pos.contains_key(draw) {
                let (row, col) = pos[draw];
                marked[row][col] = true;
                let mut is_winning = true;
                for r in 0..marked.len() {
                    if !marked[r][col] {
                        is_winning = false;
                        break;
                    }
                }
                if is_winning {
                    print_result(pos, marked, *draw);
                    return;
                }

                is_winning = true;
                for c in 0..marked[row].len() {
                    if !marked[row][c] {
                        is_winning = false;
                        break;
                    }
                }
                if is_winning {
                    print_result(pos, marked, *draw);
                    return;
                }
            }
        }
    }
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut cards: Vec<(HashMap<u32, (usize, usize)>, Vec<Vec<bool>>, bool)> = Vec::new();
    let mut last_result: u32 = 0;
    let draws: Vec<u32> = input[0].split(",").map(|x| x.parse().unwrap()).collect();

    for i in (2..input.len()).step_by(6) {
        let mut positions: HashMap<u32, (usize, usize)> = HashMap::new();

        for j in 0..5 {
            let row: Vec<u32> = input[i + j]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for k in 0..row.len() {
                positions.insert(row[k], (j, k));
            }
        }

        cards.push((positions, vec![vec![false; 5]; 5], false));
    }

    for draw in draws.iter() {
        for i in 0..cards.len() {
            let (pos, marked, has_won) = &mut cards[i];

            if *has_won {
                continue;
            }
            if pos.contains_key(draw) {
                let (row, col) = pos[draw];
                marked[row][col] = true;
                let mut is_winning = true;
                for r in 0..marked.len() {
                    if !marked[r][col] {
                        is_winning = false;
                        break;
                    }
                }
                if is_winning {
                    *has_won = true;
                    last_result = get_result(pos, marked, *draw);
                }

                is_winning = true;
                for c in 0..marked[row].len() {
                    if !marked[row][c] {
                        is_winning = false;
                        break;
                    }
                }
                if is_winning {
                    *has_won = true;
                    last_result = get_result(pos, marked, *draw);
                }
            }
        }
    }
    println!("{}", last_result);
}

fn print_result(pos: &HashMap<u32, (usize, usize)>, marked: &Vec<Vec<bool>>, draw: u32) {
    let mut sum = 0;
    for key in pos.keys() {
        let (row, col) = pos[key];
        if !marked[row][col] {
            sum += *key;
        }
    }
    println!("{}", sum * draw);
}

fn get_result(pos: &HashMap<u32, (usize, usize)>, marked: &Vec<Vec<bool>>, draw: u32) -> u32 {
    let mut sum = 0;
    for key in pos.keys() {
        let (row, col) = pos[key];
        if !marked[row][col] {
            sum += *key;
        }
    }
    sum * draw
}
