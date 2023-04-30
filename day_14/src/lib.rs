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

    let mut start = input[0].clone();

    let transformations: HashMap<&str, &str> = input
        .iter()
        .skip(2)
        .map(|x| x.split(" -> ").collect_tuple().unwrap())
        .collect();

    for _ in 0..10 {
        let chars: Vec<char> = start.chars().collect();
        let mut new: String = String::from(chars[0]);

        for i in 1..chars.len() {
            let key = String::from(chars[i - 1].to_string() + &chars[i].to_string());
            if transformations.contains_key(&key.as_str()) {
                new.push_str(transformations.get(&key.as_str()).unwrap())
            }
            new.push(chars[i]);
        }

        start = new;
    }

    let mut occurs: HashMap<char, u32> = HashMap::new();

    for c in start.chars() {
        if !occurs.contains_key(&c) {
            occurs.insert(c, 0);
        }
        let oc = occurs.get(&c).unwrap();
        occurs.insert(c, oc + 1);
    }

    let max = occurs
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();

    let min = occurs
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();

    println!("{}", *max - *min);
}

fn add_val(map: &mut HashMap<String, u64>, key: &String, inc: u64) {
    if !map.contains_key(key) {
        map.insert(key.clone(), 0);
    }
    let val = map.get(key).unwrap();
    map.insert(key.clone(), *val + inc);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());

    let start: Vec<char> = input[0].chars().collect();

    let transformations: HashMap<&str, &str> = input
        .iter()
        .skip(2)
        .map(|x| x.split(" -> ").collect_tuple().unwrap())
        .collect();

    let mut pairs: HashMap<String, u64> = HashMap::new();
    let mut chars: HashMap<char, u64> = HashMap::new();
    chars.insert(start[0], 1);

    for i in 1..start.len() {
        if !chars.contains_key(&start[i]) {
            chars.insert(start[i], 0);
        }
        let oc = chars.get(&start[i]).unwrap();
        chars.insert(start[i], *oc + 1);

        let pair = String::from(start[i - 1].to_string() + &start[i].to_string());
        add_val(&mut pairs, &pair, 1);
    }

    for _ in 0..40 {
        let mut new_pairs: HashMap<String, u64> = HashMap::new();

        for (pair, oc) in pairs.iter() {
            if transformations.contains_key(pair.as_str()) {
                let t_r = transformations
                    .get(pair.as_str())
                    .unwrap()
                    .chars()
                    .nth(0)
                    .unwrap();
                let first_pair =
                    String::from(pair.chars().nth(0).unwrap().to_string() + &t_r.to_string());
                let second_pair =
                    String::from(t_r.to_string() + &pair.chars().nth(1).unwrap().to_string());
                if !chars.contains_key(&t_r) {
                    chars.insert(t_r, 0);
                }
                let old_oc = chars.get(&t_r).unwrap();
                chars.insert(t_r, *old_oc + *oc);
                add_val(&mut new_pairs, &first_pair, *oc);
                add_val(&mut new_pairs, &second_pair, *oc);
            } else {
                add_val(&mut new_pairs, &pair, *oc);
            }
        }
        pairs = new_pairs;
    }

    let max = chars
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();

    let min = chars
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v)
        .unwrap();

    println!("{}", *max - *min);
}
