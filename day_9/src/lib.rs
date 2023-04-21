use std::fs::File;
use std::io::prelude::*;

fn get_input(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    lines
}
fn is_valid(index: i32, size: usize) -> bool {
    return index >= 0 && index < size as i32;
}

fn update_top_basins(top_basins: &mut [u32], index: usize, entry: u32) {
    for i in (index + 1..top_basins.len()).rev() {
        top_basins[i] = top_basins[i - 1];
    }
    top_basins[index] = entry;
}

fn is_valid_pos(pos: (i32, i32), size: (usize, usize)) -> bool {
    return is_valid(pos.0, size.0) && is_valid(pos.1, size.1);
}

fn traverse_grid(
    root: (i32, i32),
    matrix: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
    size: &mut u32,
) {
    if !is_valid_pos(root, (matrix.len(), matrix[0].len()))
        || matrix[root.0 as usize][root.1 as usize] == 9
        || visited[root.0 as usize][root.1 as usize]
    {
        return;
    }

    visited[root.0 as usize][root.1 as usize] = true;
    *size += 1;

    traverse_grid((root.0 - 1, root.1), matrix, visited, size);
    traverse_grid((root.0 + 1, root.1), matrix, visited, size);
    traverse_grid((root.0, root.1 - 1), matrix, visited, size);
    traverse_grid((root.0, root.1 + 1), matrix, visited, size);
}

pub fn part_2() {
    let input = get_input("input.txt".to_string());
    let matrix: Vec<Vec<u8>> = input
        .iter()
        .map(|x| x.chars().map(|x| x as u8 - '0' as u8).collect())
        .collect();

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if is_valid(x as i32 - 1, matrix.len()) && matrix[x - 1][y] <= matrix[x][y] {
                continue;
            }
            if is_valid(x as i32 + 1, matrix.len()) && matrix[x + 1][y] <= matrix[x][y] {
                continue;
            }
            if is_valid(y as i32 - 1, matrix[x].len()) && matrix[x][y - 1] <= matrix[x][y] {
                continue;
            }
            if is_valid(y as i32 + 1, matrix[x].len()) && matrix[x][y + 1] <= matrix[x][y] {
                continue;
            }

            low_points.push((x, y));
        }
    }
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut largest_basins = [0; 3];

    for low_point in low_points.iter() {
        let mut basin_size: u32 = 0;
        traverse_grid(
            (low_point.0 as i32, low_point.1 as i32),
            &matrix,
            &mut visited,
            &mut basin_size,
        );
        //println!("{}", basin_size);
        for i in 0..3 {
            if basin_size > largest_basins[i] {
                update_top_basins(&mut largest_basins, i, basin_size);
                break;
            }
        }
    }
    println!("{}", largest_basins.iter().product::<u32>());
}
pub fn part_1() {
    let input = get_input("input.txt".to_string());
    let matrix: Vec<Vec<u8>> = input
        .iter()
        .map(|x| x.chars().map(|x| x as u8 - '0' as u8).collect())
        .collect();

    let mut lowest_sum: u32 = 0;

    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            if is_valid(x as i32 - 1, matrix.len()) && matrix[x - 1][y] <= matrix[x][y] {
                continue;
            }
            if is_valid(x as i32 + 1, matrix.len()) && matrix[x + 1][y] <= matrix[x][y] {
                continue;
            }
            if is_valid(y as i32 - 1, matrix[x].len()) && matrix[x][y - 1] <= matrix[x][y] {
                continue;
            }
            if is_valid(y as i32 + 1, matrix[x].len()) && matrix[x][y + 1] <= matrix[x][y] {
                continue;
            }

            lowest_sum += matrix[x][y] as u32 + 1;
        }
    }

    println!("{}", lowest_sum);
}
