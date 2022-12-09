use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = Default::default();
    visited.insert(tail);
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let direction = split[0];
        let count: usize = split[1].parse().unwrap();
        for _ in 0..count {
            match direction {
                "U" => head.0 += 1,
                "D" => head.0 -= 1,
                "L" => head.1 -= 1,
                "R" => head.1 += 1,
                _ => unreachable!(),
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
                visited.insert(tail);
            }
        }
    }
    println!("{}", visited.len());
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let length = 10;
    let mut rope = vec![(0, 0); length];
    let mut visited: HashSet<(i32, i32)> = Default::default();
    visited.insert(*rope.last().unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        let direction = split[0];
        let count: usize = split[1].parse().unwrap();
        for _ in 0..count {
            match direction {
                "U" => rope[0].0 += 1,
                "D" => rope[0].0 -= 1,
                "L" => rope[0].1 -= 1,
                "R" => rope[0].1 += 1,
                _ => unreachable!(),
            }
            for i in 1..rope.len() {
                if (rope[i - 1].0 - rope[i].0).abs() > 1 || (rope[i - 1].1 - rope[i].1).abs() > 1 {
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    println!("{}", visited.len());
}
