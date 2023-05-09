use std::fs::File;
use std::io::prelude::*;

use snailfish::Snailfish;
pub mod snailfish;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

pub fn part_1() {
    let input: Vec<Snailfish> = get_input("input.txt".to_string())
        .iter()
        .map(|x| Snailfish::from_text(x))
        .collect();

    let mut sum = input[0].clone();
    for snailfish in input[1..].iter() {
        //sum.to_text();
        // println!();
        sum = sum.add(snailfish);
        sum.reduce();
    }
    println!("{}", sum.magnitude())
}

pub fn part_2() {
    let input: Vec<Snailfish> = get_input("input.txt".to_string())
        .iter()
        .map(|x| Snailfish::from_text(x))
        .collect();

    let mut max = 0u64;
    for first in 0..input.len() {
        for second in 0..input.len() {
            if first == second {
                continue;
            }
            let mut sum = input[first].add(&input[second]);
            sum.reduce();
            max = max.max(sum.magnitude());
        }
    }
    println!("{}", max);
}

pub fn test_snail() {
    let mut snailfish = Snailfish::from_text("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let other_snailfish = Snailfish::from_text("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
    snailfish = snailfish.add(&other_snailfish);

    snailfish.reduce();
}
