use std::collections::{HashMap, HashSet};
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

fn add_edge<'a>(graph: &mut HashMap<&'a str, Vec<&'a str>>, first: &'a str, second: &'a str) {
    if !graph.contains_key(first) {
        graph.insert(first, Vec::new());
    }

    graph.get_mut(first).unwrap().push(second);
}

fn traverse_grid(
    node: &str,
    grid: &HashMap<&str, Vec<&str>>,
    taken: &HashMap<&str, bool>,
    cache: &mut HashSet<String>,
    parent_path: &str,
    paths: &mut u32,
) {
    if node == "end" {
        *paths += 1;
        return;
    }

    let mut current_path: String = String::from(parent_path);
    current_path.push_str(",");
    current_path.push_str(node);

    if *taken.get(node).unwrap() || cache.contains(&current_path) {
        return;
    }

    let c: char = node.chars().nth(0).unwrap();
    let mut new_taken = taken.clone();

    if c >= 'a' && c <= 'z' {
        new_taken.insert(node, true);
    }
    cache.insert(current_path.clone());

    for neighbour in grid.get(node).unwrap().iter() {
        traverse_grid(neighbour, grid, &new_taken, cache, &current_path, paths)
    }
}

fn traverse_grid_mod(
    node: &str,
    grid: &HashMap<&str, Vec<&str>>,
    taken: &HashMap<&str, bool>,
    cache: &mut HashSet<String>,
    parent_path: &str,
    exception: &mut String,
    paths: &mut HashSet<String>,
) {
    let mut current_path: String = String::from(parent_path);
    current_path.push_str(",");
    current_path.push_str(node);

    if node == "end" {
        paths.insert(current_path);
        return;
    }

    if *taken.get(node).unwrap() || cache.contains(&current_path) {
        return;
    }

    let c: char = node.chars().nth(0).unwrap();
    let mut new_taken = taken.clone();

    if c >= 'a' && c <= 'z' {
        if node == exception {
            *exception = String::from("");
        } else {
            new_taken.insert(node, true);
        }
    }
    cache.insert(current_path.clone());

    for neighbour in grid.get(node).unwrap().iter() {
        traverse_grid_mod(
            neighbour,
            grid,
            &new_taken,
            cache,
            &current_path,
            &mut exception.clone(),
            paths,
        )
    }
}

pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.iter() {
        let nodes: (&str, &str) = line.split("-").collect_tuple().unwrap();
        add_edge(&mut graph, nodes.0, nodes.1);
        add_edge(&mut graph, nodes.1, nodes.0);
    }

    let mut taken: HashMap<&str, bool> = HashMap::new();
    graph.keys().for_each(|x| {
        taken.insert(*x, false);
    });

    let mut cache: HashSet<String> = HashSet::new();
    let mut paths: u32 = 0;

    traverse_grid("start", &graph, &taken, &mut cache, "", &mut paths);

    println!("{}", paths);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.iter() {
        let nodes: (&str, &str) = line.split("-").collect_tuple().unwrap();
        add_edge(&mut graph, nodes.0, nodes.1);
        add_edge(&mut graph, nodes.1, nodes.0);
    }
    let mut paths: HashSet<String> = HashSet::new();

    for key in graph
        .keys()
        .filter(|x| x.chars().nth(0).unwrap().is_lowercase() && **x != "end" && **x != "start")
    {
        let mut taken: HashMap<&str, bool> = HashMap::new();
        graph.keys().for_each(|x| {
            taken.insert(*x, false);
        });
        let mut cache: HashSet<String> = HashSet::new();
        let mut k = key.to_string();
        traverse_grid_mod("start", &graph, &taken, &mut cache, "", &mut k, &mut paths);
    }

    println!("{}", paths.len());
}
