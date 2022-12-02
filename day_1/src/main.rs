use std::{
    cmp::Reverse,
    collections::BinaryHeap,
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
    let mut max = 0;
    let mut current = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            max = max.max(current);
            current = 0;
        } else {
            let num = line.parse::<i64>().unwrap();
            current += num;
        }
    }
    max = max.max(current);

    println!("{:?}", max);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut max_3 = BinaryHeap::new();
    let mut current = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if max_3.len() < 3 {
                max_3.push(Reverse(current));
            } else if max_3.peek().unwrap().0 < current {
                max_3.pop();
                max_3.push(Reverse(current));
            }
            current = 0;
        } else {
            let num = line.parse::<i64>().unwrap();
            current += num;
        }
    }

    if max_3.len() < 3 {
        max_3.push(Reverse(current));
    } else if max_3.peek().unwrap().0 < current {
        max_3.pop();
        max_3.push(Reverse(current));
    }

    let sum: i64 = max_3.iter().map(|x| x.0).sum();

    println!("{:?}", sum);
}
