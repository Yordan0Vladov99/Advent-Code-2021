use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}

fn is_valid(coord: i32, limit: usize) -> bool {
    return coord >= 0 && coord < limit as i32;
}

pub fn part_1() {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let input = get_input("input.txt".to_string());
    for line in input.iter() {
        grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    let mut q: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();

    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();

    let mut dist: HashMap<(i32, i32), u32> = HashMap::new();
    dist.insert((0, 0), 0);
    q.push((0, 0), 0);

    while !q.is_empty() {
        let p = q.pop().unwrap();
        let (row, col) = (p.0 .0 as i32, p.0 .1 as i32);

        for (r, c) in [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .iter()
        {
            if !is_valid(*r, rows) || !is_valid(*c, cols) {
                continue;
            }

            let d = dist.get(&(row, col)).unwrap() + grid[*r as usize][*c as usize];
            if !dist.contains_key(&(*r, *c)) || *dist.get(&(*r, *c)).unwrap() > d {
                dist.insert((*r, *c), d);
                q.push((*r, *c), d as i32 * -1);
            }
        }
    }

    println!("{}", dist.get(&(rows as i32 - 1, cols as i32 - 1)).unwrap())
}

pub fn part_2() {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let input = get_input("input.txt".to_string());
    for line in input.iter() {
        grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    for i in 0..grid.len() {
        let row_len = grid[i].len() * 4;
        for j in 0..row_len {
            let val = match grid[i][j] {
                9 => 1,
                _ => grid[i][j] + 1,
            };
            grid[i].push(val);
        }
    }

    let grid_len = grid.len() * 4;

    for i in 0..grid_len {
        let new_row: Vec<u32> = grid[i]
            .iter()
            .map(|x| match x {
                9 => 1,
                _ => x + 1,
            })
            .collect();
        grid.push(new_row);
    }

    let mut q: PriorityQueue<(i32, i32), i32> = PriorityQueue::new();

    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();

    let mut dist: HashMap<(i32, i32), u32> = HashMap::new();
    dist.insert((0, 0), 0);
    q.push((0, 0), 0);

    while !q.is_empty() {
        let p = q.pop().unwrap();
        let (row, col) = (p.0 .0 as i32, p.0 .1 as i32);

        for (r, c) in [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]
        .iter()
        {
            if !is_valid(*r, rows) || !is_valid(*c, cols) {
                continue;
            }

            let d = dist.get(&(row, col)).unwrap() + grid[*r as usize][*c as usize];
            if !dist.contains_key(&(*r, *c)) || *dist.get(&(*r, *c)).unwrap() > d {
                dist.insert((*r, *c), d);
                q.push((*r, *c), d as i32 * -1);
            }
        }
    }

    println!("{}", dist.get(&(rows as i32 - 1, cols as i32 - 1)).unwrap())
}
