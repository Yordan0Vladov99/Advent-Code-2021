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
    let line = &input[0];

    let target_min_y: i32 = line
        .split(", ")
        .nth(1)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split("..")
        .nth(0)
        .unwrap()
        .parse()
        .unwrap();
    let n = -1 * target_min_y - 1;
    println!("{}", n * (n + 1) / 2);
}

fn hits_area(x_min: i32, x_max: i32, y_min: i32, y_max: i32, mut vx: i32, mut vy: i32) -> bool {
    let (mut x, mut y) = (0, 0);

    loop {
        if x > x_max {
            return false;
        }

        if vx == 0 && !(x >= x_min && x <= x_max) {
            return false;
        }

        if vx == 0 && y < y_min {
            return false;
        }

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return true;
        }

        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
    }
}
pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let line = &input[0];
    let equations: Vec<&str> = line.split(": ").nth(1).unwrap().split(", ").collect();
    let limits: Vec<Vec<i32>> = equations
        .iter()
        .map(|eq| {
            eq.split("=")
                .nth(1)
                .unwrap()
                .split("..")
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let (min_x, max_x, min_y, max_y) = (limits[0][0], limits[0][1], limits[1][0], limits[1][1]);
    let max_y_val = if min_y.abs() > max_y.abs() {
        min_y.abs()
    } else {
        max_y.abs()
    };

    let mut distinct_pos = 0u32;

    for x in 0..=max_x {
        for y in max_y_val * -1..=max_y_val {
            distinct_pos += hits_area(min_x, max_x, min_y, max_y, x, y) as u32;
        }
    }

    println!("{}", distinct_pos);
}
