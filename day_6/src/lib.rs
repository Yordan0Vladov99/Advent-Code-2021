use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn calc_fish(iters: usize) {
    let mut fish: Vec<u64> = vec![0; 9];

    get_input("input.txt".to_string())[0]
        .split(",")
        .for_each(|x| fish[x.parse::<usize>().unwrap()] += 1);
    for _ in 0..iters {
        let mut new_fish: Vec<u64> = vec![0; 9];
        new_fish[6] += fish[0];
        new_fish[8] += fish[0];

        for i in 0..8 {
            new_fish[i] += fish[i + 1];
        }
        fish = new_fish;
    }
    println!("{}", fish.iter().sum::<u64>());
}

pub fn part_1() {
    calc_fish(80);
}

pub fn part_2() {
    calc_fish(256);
}
