use std::{
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
    let mut points = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        points += match split[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unimplemented!(),
        };
        points += match line.as_str() {
            "A X" | "B Y" | "C Z" => 3,
            "A Y" | "B Z" | "C X" => 6,
            _ => 0,
        }
    }

    println!("{:?}", points);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut points = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split_whitespace().collect();
        points += match split[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => unimplemented!(),
        };
        points += match line.as_str() {
            "A X" => 3,
            "A Y" => 1,
            "A Z" => 2,
            "B X" => 1,
            "B Y" => 2,
            "B Z" => 3,
            "C X" => 2,
            "C Y" => 3,
            "C Z" => 1,
            _ => unimplemented!(),
        }
    }

    println!("{:?}", points);
}
