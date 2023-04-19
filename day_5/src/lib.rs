use std::cmp;
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
    let input = get_input("input.txt".to_string());
    let mut occurs: HashMap<(u32, u32), u32> = HashMap::new();
    for line in input.iter() {
        let pairs: Vec<(u32, u32)> = line
            .split(" -> ")
            .map(|p| {
                p.split(",")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        if pairs[0].0 == pairs[1].0 {
            let min = cmp::min(pairs[0].1, pairs[1].1);
            let max = cmp::max(pairs[0].1, pairs[1].1);

            for i in min..=max {
                let pos = (pairs[0].0, i);

                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        } else if pairs[0].1 == pairs[1].1 {
            let min = cmp::min(pairs[0].0, pairs[1].0);
            let max = cmp::max(pairs[0].0, pairs[1].0);

            for i in min..=max {
                let pos = (i, pairs[0].1);

                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        }
    }
    let mut overlaps = 0;
    occurs.values().for_each(|x| {
        if *x >= 2 {
            overlaps += 1;
        }
    });

    println!("{}", overlaps);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut occurs: HashMap<(i32, i32), u32> = HashMap::new();
    for line in input.iter() {
        let pairs: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|p| {
                p.split(",")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        if pairs[0].0 == pairs[1].0 {
            let min = cmp::min(pairs[0].1, pairs[1].1);
            let max = cmp::max(pairs[0].1, pairs[1].1);

            for i in min..=max {
                let pos = (pairs[0].0, i);

                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        } else if pairs[0].1 == pairs[1].1 {
            let min = cmp::min(pairs[0].0, pairs[1].0);
            let max = cmp::max(pairs[0].0, pairs[1].0);

            for i in min..=max {
                let pos = (i, pairs[0].1);

                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        } else if pairs[0].0.abs_diff(pairs[0].1) == pairs[1].0.abs_diff(pairs[1].1) {
            let x_dir: i32 = if pairs[0].0 > pairs[1].0 { -1 } else { 1 };
            let y_dir: i32 = if pairs[0].1 > pairs[1].1 { -1 } else { 1 };

            for i in 0..=pairs[0].0.abs_diff(pairs[1].0) {
                let pos = (pairs[0].0 + x_dir * i as i32, pairs[0].1 + y_dir * i as i32);
                //println!("{} {}", pos.0, pos.1);
                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        } else if pairs[0].0 + pairs[0].1 == pairs[1].0 + pairs[1].1 {
            let x_dir: i32 = if pairs[0].0 > pairs[1].0 { -1 } else { 1 };
            let y_dir: i32 = if pairs[0].1 > pairs[1].1 { -1 } else { 1 };

            for i in 0..=pairs[0].0.abs_diff(pairs[1].0) {
                let pos = (pairs[0].0 + x_dir * i as i32, pairs[0].1 + y_dir * i as i32);
                //println!("{} {}", pos.0, pos.1);
                if !occurs.contains_key(&pos) {
                    occurs.insert(pos, 1);
                } else {
                    occurs.insert(pos, occurs[&pos] + 1);
                }
            }
        }
    }
    let mut overlaps = 0;
    occurs.values().for_each(|x| {
        if *x >= 2 {
            overlaps += 1;
        }
    });

    println!("{}", overlaps);
}
