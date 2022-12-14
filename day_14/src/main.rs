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
    let mut blocked: HashSet<(i32, i32)> = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let corners: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|coord| {
                let mut pos = coord.split(',');
                (
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect();

        for pair in corners.windows(2) {
            let mut current = pair[0];
            while current != pair[1] {
                blocked.insert(current);
                if current.0 == pair[1].0 {
                    current.1 += (pair[1].1 - current.1).signum();
                } else if current.1 == pair[1].1 {
                    current.0 += (pair[1].0 - current.0).signum();
                } else {
                    unreachable!();
                }
            }
            blocked.insert(pair[1]);
        }
    }

    let mut resting = 0;
    let max_depth = *blocked.iter().map(|(_, d)| d).max().unwrap();
    loop {
        let mut current = (500, 0);
        let mut stopped = false;
        while !stopped {
            if current.1 == max_depth {
                println!("{}", resting);
                return;
            } else if !blocked.contains(&(current.0, current.1 + 1)) {
                current = (current.0, current.1 + 1);
            } else if !blocked.contains(&(current.0 - 1, current.1 + 1)) {
                current = (current.0 - 1, current.1 + 1);
            } else if !blocked.contains(&(current.0 + 1, current.1 + 1)) {
                current = (current.0 + 1, current.1 + 1);
            } else {
                blocked.insert(current);
                resting += 1;
                stopped = true;
            }
        }
    }
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut blocked: HashSet<(i32, i32)> = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let corners: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|coord| {
                let mut pos = coord.split(',');
                (
                    pos.next().unwrap().parse::<i32>().unwrap(),
                    pos.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect();

        for pair in corners.windows(2) {
            let mut current = pair[0];
            while current != pair[1] {
                blocked.insert(current);
                if current.0 == pair[1].0 {
                    current.1 += (pair[1].1 - current.1).signum();
                } else if current.1 == pair[1].1 {
                    current.0 += (pair[1].0 - current.0).signum();
                } else {
                    unreachable!();
                }
            }
            blocked.insert(pair[1]);
        }
    }

    let mut resting = 0;
    let floor = *blocked.iter().map(|(_, d)| d).max().unwrap() + 2;
    loop {
        let mut current = (500, 0);
        let mut stopped = false;
        while !stopped {
            if current.1 + 1 == floor {
                blocked.insert(current);
                resting += 1;
                stopped = true;
                if current == (500, 0) {
                    println!("{}", resting);
                    return;
                }
            } else if !blocked.contains(&(current.0, current.1 + 1)) {
                current = (current.0, current.1 + 1);
            } else if !blocked.contains(&(current.0 - 1, current.1 + 1)) {
                current = (current.0 - 1, current.1 + 1);
            } else if !blocked.contains(&(current.0 + 1, current.1 + 1)) {
                current = (current.0 + 1, current.1 + 1);
            } else {
                blocked.insert(current);
                resting += 1;
                stopped = true;
                if current == (500, 0) {
                    println!("{}", resting);
                    return;
                }
            }
        }
    }
}
