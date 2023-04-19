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
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.iter() {
        let args: Vec<&str> = line.split(" ").collect();
        let val: i32 = args[1].parse().unwrap();

        match args[0] {
            "forward" => horizontal += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => {}
        }
    }
    println!("{}", horizontal * depth);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.iter() {
        let args: Vec<&str> = line.split(" ").collect();
        let val: i32 = args[1].parse().unwrap();

        match args[0] {
            "forward" => {
                horizontal += val;
                depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => {}
        }
    }
    println!("{}", horizontal * depth);
}
