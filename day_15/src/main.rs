use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_1();
    part_2();
}

fn parse_eq_plus(eq: &str) -> i64 {
    parse_eq(&eq[..eq.len() - 1])
}

fn parse_eq(eq: &str) -> i64 {
    eq.split('=').last().unwrap().parse().unwrap()
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut blocked: HashSet<i64> = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();
        let sx = parse_eq_plus(split[2]);
        let sy = parse_eq_plus(split[3]);
        let bx = parse_eq_plus(split[8]);
        let by = parse_eq(split[9]);
        let distance = (sx - bx).abs() + (sy - by).abs();
        let y = 2000000;
        for x in (sx - distance + (y - sy).abs())..=(sx + distance - (y - sy).abs()) {
            if (x, y) != (bx, by) {
                blocked.insert(x);
            }
        }
    }
    println!("{}", blocked.len());
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let limit: i64 = 4000000;
    let mut sensors: Vec<(i64, i64, i64)> = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();
        let sx = parse_eq_plus(split[2]);
        let sy = parse_eq_plus(split[3]);
        let bx = parse_eq_plus(split[8]);
        let by = parse_eq(split[9]);
        let distance = (sx - bx).abs() + (sy - by).abs();
        sensors.push((sx, sy, distance));
    }
    let mut x = 0;
    while x <= limit {
        let mut y = 0;
        while y <= limit {
            let mut max_gap = -1;
            for (sx, sy, distance) in sensors.iter() {
                max_gap = max_gap.max(distance - (x - sx).abs() - (y - sy).abs());
            }
            if max_gap == -1 {
                println!("{}", x * limit + y);
                return;
            }
            y += max_gap + 1;
        }
        x += 1;
    }
    println!("fail");
}
