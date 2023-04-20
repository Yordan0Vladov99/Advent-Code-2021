use std::collections::HashMap;
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
    let lines = get_input("input.txt".to_string());
    let mut valid_nums = 0;
    for line in lines.iter() {
        let args: Vec<&str> = line.split(" ").collect();

        for i in 11..args.len() {
            match args[i].len() {
                2 | 3 | 4 | 7 => {
                    valid_nums += 1;
                }
                _ => {}
            }
        }
    }
    println!("{}", valid_nums);
}

fn contains_feature(digit: &str, feature: &str) -> bool {
    let mut contains = true;
    feature.chars().for_each(|x| contains &= digit.contains(x));
    contains
}

pub fn part_2() {
    let lines = get_input("input.txt".to_string());
    let mut sum = 0;
    for line in lines.iter() {
        let mut digits: HashMap<&str, u32> = HashMap::new();
        let args: Vec<&str> = line.split(" ").collect();
        let mut sorted_args: Vec<String> = args[0..10]
            .iter()
            .map(|x| x.chars().sorted().collect())
            .collect();
        sorted_args.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        //println!("{}", sorted_args.join(" "));

        digits.insert(&sorted_args[0], 1);
        digits.insert(&sorted_args[1], 7);
        digits.insert(&sorted_args[2], 4);
        digits.insert(&sorted_args[9], 8);

        let right_wall = sorted_args[0].to_string();
        let left_arch = sorted_args[2]
            .chars()
            .filter(|x| !right_wall.contains(*x))
            .collect::<String>();
        for i in 3..9 {
            let arg = &sorted_args[i];
            if contains_feature(arg, &right_wall) && !contains_feature(arg, &left_arch) {
                if arg.len() == 5 {
                    digits.insert(arg, 3);
                } else if arg.len() == 6 {
                    digits.insert(arg, 0);
                }
            } else if contains_feature(arg, &right_wall) && contains_feature(arg, &left_arch) {
                digits.insert(arg, 9);
            } else if !contains_feature(arg, &right_wall) && !contains_feature(arg, &left_arch) {
                digits.insert(arg, 2);
            } else if arg.len() == 5 {
                digits.insert(arg, 5);
            } else if arg.len() == 6 {
                digits.insert(arg, 6);
            }
        }

        let sorted_output: Vec<String> = args[11..]
            .iter()
            .map(|x| x.chars().sorted().collect())
            .collect();
        let mut num = 0;
        for n in sorted_output.iter() {
            num *= 10;
            num += digits.get(n.as_str()).unwrap();
        }
        sum += num;
    }

    println!("{}", sum);
}
