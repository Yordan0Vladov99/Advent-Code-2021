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
    let nums: Vec<u32> = get_input("input.txt".to_string())[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut min: Option<u32> = None;

    for i in 0..nums.len() {
        let mut sum = 0;
        for j in 0..nums.len() {
            sum += nums[i].abs_diff(nums[j]);
        }
        match min {
            None => {
                min = Some(sum);
            }
            Some(x) => {
                if sum < x {
                    min = Some(sum);
                }
            }
        }
    }

    println!("{}", min.unwrap());
}

pub fn part_2() {
    let nums: Vec<u32> = get_input("input.txt".to_string())[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut min: Option<u32> = None;

    for i in 0..nums.len() {
        let mut sum = 0;
        for j in 0..nums.len() {
            let diff = nums[i].abs_diff(nums[j]);
            sum += diff * (diff + 1) / 2;
        }
        match min {
            None => {
                min = Some(sum);
            }
            Some(x) => {
                if sum < x {
                    min = Some(sum);
                }
            }
        }
    }

    println!("{}", min.unwrap());
}
