use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_1() {
    let positions: HashSet<(u32, u32)> = get_input("positions.txt".to_string())
        .iter()
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let instructions: Vec<(char, u32)> = get_input("instructions.txt".to_string())
        .iter()
        .map(|x| {
            let args: Vec<&str> = x.split(" ").collect();
            let (axis, pos): (&str, &str) = args[2].split("=").collect_tuple().unwrap();

            (axis.chars().nth(0).unwrap(), pos.parse::<u32>().unwrap())
        })
        .collect();

    let (axis, fold) = instructions.get(0).unwrap();
    let mut new_positions: HashSet<(u32, u32)> = HashSet::new();
    for pos in positions.iter() {
        let mut new_pos: (u32, u32) = (pos.0, pos.1);
        if *axis == 'x' && pos.0 > *fold {
            new_pos.0 = 2 * fold - pos.0;
        } else if *axis == 'y' && pos.1 > *fold {
            new_pos.1 = 2 * fold - pos.1;
        }
        new_positions.insert(new_pos);
    }
    println!("{}", new_positions.len());
}

pub fn part_2() {
    let mut positions: HashSet<(u32, u32)> = get_input("positions.txt".to_string())
        .iter()
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let instructions: Vec<(char, u32)> = get_input("instructions.txt".to_string())
        .iter()
        .map(|x| {
            let args: Vec<&str> = x.split(" ").collect();
            let (axis, pos): (&str, &str) = args[2].split("=").collect_tuple().unwrap();

            (axis.chars().nth(0).unwrap(), pos.parse::<u32>().unwrap())
        })
        .collect();

    for (axis, fold) in instructions.iter() {
        let mut new_positions: HashSet<(u32, u32)> = HashSet::new();
        for pos in positions.iter() {
            let mut new_pos: (u32, u32) = (pos.0, pos.1);
            if *axis == 'x' && pos.0 > *fold {
                new_pos.0 = 2 * fold - pos.0;
            } else if *axis == 'y' && pos.1 > *fold {
                new_pos.1 = 2 * fold - pos.1;
            }
            new_positions.insert(new_pos);
        }
        positions = new_positions;
    }

    let max_x = positions.iter().map(|x| x.0).max().unwrap() + 1;
    let max_y = positions.iter().map(|x| x.1).max().unwrap() + 1;

    for y in 0..max_y {
        for x in 0..max_x {
            match positions.contains(&(x, y)) {
                true => print!("#"),
                false => print!(" "),
            }
        }
        println!();
    }
}
