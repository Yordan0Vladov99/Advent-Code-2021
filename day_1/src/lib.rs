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
    let depths: Vec<u32> = get_input("input.txt".to_string())
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut increases = 0;
    for i in 0..depths.len() - 1 {
        if depths[i] < depths[i + 1] {
            increases += 1;
        }
    }
    println!("{}", increases);
}

pub fn part_2() {
    let depths: Vec<u32> = get_input("input.txt".to_string())
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut increases = 0;
    for i in 0..depths.len() - 3 {
        if depths[i] + depths[i + 1] + depths[i + 2] < depths[i + 1] + depths[i + 2] + depths[i + 3]
        {
            increases += 1;
        }
    }
    println!("{}", increases);
}
