use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn hex_to_binary<'a>(hex: char) -> &'a str {
    match hex {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn hex_to_binary_string(hex: &str) -> String {
    return hex
        .chars()
        .map(hex_to_binary)
        .collect::<Vec<&str>>()
        .join("");
}

fn read_literal(input: &str, i: &mut usize) -> u64 {
    let mut res = String::new();
    while &input[*i..*i + 1] != "0" {
        res.push_str(&input[*i + 1..=*i + 4]);
        *i += 5;
    }

    res.push_str(&input[*i + 1..=*i + 4]);

    *i += 5;

    return u64::from_str_radix(&res, 2).unwrap();
}

fn read_by_total_size(input: &str, i: &mut usize, version_sum: &mut u64, operation: &str) -> u64 {
    *i += 1;
    let size = usize::from_str_radix(&input[*i..*i + 15], 2).unwrap();
    *i += 15;

    match operation {
        "+" => {
            let start = *i;
            let mut sum = 0u64;
            while *i < start + size {
                sum += read_packet(input, i, version_sum);
            }
            sum
        }
        "*" => {
            let start = *i;
            let mut product = 1u64;
            while *i < start + size {
                product *= read_packet(input, i, version_sum);
            }
            product
        }
        "m" => {
            let start = *i;
            let mut min: Option<u64> = None;
            while *i < start + size {
                let packet_val = read_packet(input, i, version_sum);
                if let Some(v) = min {
                    if v > packet_val {
                        min = Some(packet_val);
                    }
                } else {
                    min = Some(packet_val);
                }
            }
            min.unwrap()
        }
        "M" => {
            let start = *i;
            let mut max: Option<u64> = None;
            while *i < start + size {
                let packet_val = read_packet(input, i, version_sum);
                if let Some(v) = max {
                    if v < packet_val {
                        max = Some(packet_val);
                    }
                } else {
                    max = Some(packet_val);
                }
            }
            max.unwrap()
        }
        ">" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first > second) as u64
        }
        "<" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first < second) as u64
        }
        "=" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first == second) as u64
        }
        _ => 0,
    }
}
fn read_by_number_of_packets(
    input: &str,
    i: &mut usize,
    version_sum: &mut u64,
    operation: &str,
) -> u64 {
    *i += 1;
    let operations = usize::from_str_radix(&input[*i..*i + 11], 2).unwrap();
    *i += 11;

    match operation {
        "+" => {
            let mut sum = 0u64;
            for _ in 0..operations {
                sum += read_packet(input, i, version_sum);
            }
            sum
        }
        "*" => {
            let mut product = 1u64;
            for _ in 0..operations {
                product *= read_packet(input, i, version_sum);
            }
            product
        }
        "m" => {
            let mut min: Option<u64> = None;
            for _ in 0..operations {
                let packet_val = read_packet(input, i, version_sum);
                if let Some(v) = min {
                    if v > packet_val {
                        min = Some(packet_val);
                    }
                } else {
                    min = Some(packet_val);
                }
            }
            min.unwrap()
        }
        "M" => {
            let mut max: Option<u64> = None;
            for _ in 0..operations {
                let packet_val = read_packet(input, i, version_sum);
                if let Some(v) = max {
                    if v < packet_val {
                        max = Some(packet_val);
                    }
                } else {
                    max = Some(packet_val);
                }
            }
            max.unwrap()
        }
        ">" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first > second) as u64
        }
        "<" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first < second) as u64
        }
        "=" => {
            let first = read_packet(input, i, version_sum);
            let second = read_packet(input, i, version_sum);

            (first == second) as u64
        }
        _ => 0,
    }
}

fn read_operation(input: &str, i: &mut usize, version_sum: &mut u64, operation: &str) -> u64 {
    match &input[*i..*i + 1] {
        "0" => read_by_total_size(input, i, version_sum, operation),
        "1" => read_by_number_of_packets(input, i, version_sum, operation),
        _ => 0u64,
    }
}

fn read_packet(binary_input: &str, i: &mut usize, version_sum: &mut u64) -> u64 {
    let version = &binary_input[*i..=*i + 2];
    let version_val = u64::from_str_radix(version, 2).unwrap();
    *version_sum += version_val;
    let type_id = &binary_input[*i + 3..=*i + 5];
    *i += 6;
    match type_id {
        "100" => read_literal(&binary_input, i),
        "000" => read_operation(&binary_input, i, version_sum, "+"),
        "001" => read_operation(&binary_input, i, version_sum, "*"),
        "010" => read_operation(&binary_input, i, version_sum, "m"),
        "011" => read_operation(&binary_input, i, version_sum, "M"),
        "101" => read_operation(&binary_input, i, version_sum, ">"),
        "110" => read_operation(&binary_input, i, version_sum, "<"),
        "111" => read_operation(&binary_input, i, version_sum, "="),
        _ => read_operation(&binary_input, i, version_sum, ""),
    }
}

pub fn part_1() {
    let input = get_input("input.txt".to_string())
        .get(0)
        .unwrap()
        .to_owned();
    let binary_input = hex_to_binary_string(&input);

    let mut version_sum = 0u64;

    let mut i = 0;

    read_packet(&binary_input, &mut i, &mut version_sum);

    println!("{version_sum}")
}

pub fn part_2() {
    let input = get_input("input.txt".to_string())
        .get(0)
        .unwrap()
        .to_owned();
    let binary_input = hex_to_binary_string(&input);

    let mut version_sum = 0u64;

    let mut i = 0;

    println!("{}", read_packet(&binary_input, &mut i, &mut version_sum));
}
