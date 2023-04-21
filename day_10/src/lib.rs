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
    let mut error_sum = 0;
    for line in input.iter() {
        let brackets: Vec<char> = line.chars().collect();
        let mut validator: Vec<char> = Vec::new();

        for bracket in brackets.iter() {
            match bracket {
                '{' | '[' | '(' | '<' => {
                    validator.push(*bracket);
                }
                '}' | ']' | ')' | '>' => {
                    let top = validator.pop().unwrap();
                    if (top == '{' && *bracket != '}')
                        || (top == '[' && *bracket != ']')
                        || (top == '(' && *bracket != ')')
                        || (top == '<' && *bracket != '>')
                    {
                        error_sum += match bracket {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    println!("{error_sum}");
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut scores: Vec<u64> = Vec::new();
    for line in input.iter() {
        let brackets: Vec<char> = line.chars().collect();
        let mut validator: Vec<char> = Vec::new();
        let mut is_valid = true;
        for bracket in brackets.iter() {
            match bracket {
                '{' | '[' | '(' | '<' => {
                    validator.push(*bracket);
                }
                '}' | ']' | ')' | '>' => {
                    let top = validator.pop().unwrap();
                    if (top == '{' && *bracket != '}')
                        || (top == '[' && *bracket != ']')
                        || (top == '(' && *bracket != ')')
                        || (top == '<' && *bracket != '>')
                    {
                        is_valid = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !is_valid {
            continue;
        }
        let mut score: u64 = 0;
        while !validator.is_empty() {
            score *= 5;
            score += match validator.pop().unwrap() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        scores.push(score);
    }

    scores.sort();

    println!("{}", scores[scores.len() / 2]);
}
