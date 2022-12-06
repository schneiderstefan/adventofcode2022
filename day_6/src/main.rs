use std::{
    collections::HashMap,
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
    let line: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();
    let mut last_chars: HashMap<char, u32> = Default::default();
    for (i, c) in line.iter().enumerate() {
        if last_chars.contains_key(c) {
            *last_chars.get_mut(c).unwrap() += 1;
        } else {
            last_chars.insert(*c, 1);
        }

        if i >= 4 {
            let dropping = line[i - 4];
            *last_chars.get_mut(&dropping).unwrap() -= 1;
            if last_chars[&dropping] == 0 {
                last_chars.remove(&dropping);
            }
        }
        if last_chars.len() == 4 {
            println!("{}", i + 1);
            return;
        }
    }

    unreachable!();
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let line: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();
    let mut last_chars: HashMap<char, u32> = Default::default();
    for (i, c) in line.iter().enumerate() {
        if last_chars.contains_key(c) {
            *last_chars.get_mut(c).unwrap() += 1;
        } else {
            last_chars.insert(*c, 1);
        }

        if i >= 14 {
            let dropping = line[i - 14];
            *last_chars.get_mut(&dropping).unwrap() -= 1;
            if last_chars[&dropping] == 0 {
                last_chars.remove(&dropping);
            }
        }
        if last_chars.len() == 14 {
            println!("{}", i + 1);
            return;
        }
    }

    unreachable!();
}
