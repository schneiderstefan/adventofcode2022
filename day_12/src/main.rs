use ndarray::Array2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_1();
    part_2();
}

fn neighbours(s: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = s;
    let x = x as i32;
    let y = y as i32;
    let neighbours: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    neighbours
        .into_iter()
        .map(|(a, b)| ((a + x) as usize, (b + y) as usize))
        .collect()
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let heights: Vec<i32> = lines
        .iter()
        .flat_map(|l| {
            l.chars().map(|c| {
                if c == 'S' {
                    0
                } else if c == 'E' {
                    'z' as i32 - 'a' as i32
                } else {
                    c as i32 - 'a' as i32
                }
            })
        })
        .collect();
    let shape = (lines.len(), heights.len() / lines.len());
    let grid = Array2::from_shape_vec(shape, heights).unwrap();
    let mut visited = Array2::from_elem(shape, false);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
        }
    }

    let mut layer = vec![start];
    let mut next_layer = vec![];
    let mut depth = 0;
    while !layer.is_empty() {
        for pos in &layer {
            let pos = *pos;
            if !visited[pos] {
                if pos == end {
                    println!("{}", depth);
                    return;
                }
                visited[pos] = true;
                for n in neighbours(pos) {
                    if let Some(height) = grid.get(n) {
                        if (height - grid[pos]) <= 1 && !visited[n] {
                            next_layer.push(n);
                        }
                    }
                }
            }
        }
        depth += 1;
        std::mem::swap(&mut layer, &mut next_layer);
        next_layer = vec![];
    }
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let heights: Vec<i32> = lines
        .iter()
        .flat_map(|l| {
            l.chars().map(|c| {
                if c == 'S' {
                    0
                } else if c == 'E' {
                    'z' as i32 - 'a' as i32
                } else {
                    c as i32 - 'a' as i32
                }
            })
        })
        .collect();
    let shape = (lines.len(), heights.len() / lines.len());
    let grid = Array2::from_shape_vec(shape, heights).unwrap();
    let mut visited = Array2::from_elem(shape, false);
    let mut end = (0, 0);
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'E' {
                end = (i, j);
            }
        }
    }

    let mut layer = vec![end];
    let mut next_layer = vec![];
    let mut depth = 0;
    while !layer.is_empty() {
        for pos in &layer {
            let pos = *pos;
            if !visited[pos] {
                if grid[pos] == 0 {
                    println!("{}", depth);
                    return;
                }
                visited[pos] = true;
                for n in neighbours(pos) {
                    if let Some(height) = grid.get(n) {
                        if (height - grid[pos]) >= -1 && !visited[n] {
                            next_layer.push(n);
                        }
                    }
                }
            }
        }
        depth += 1;
        std::mem::swap(&mut layer, &mut next_layer);
        next_layer = vec![];
    }
}
