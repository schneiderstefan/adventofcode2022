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
    let mut value = 1;
    let mut sum = 0;
    let mut cycles = 1;
    for line in reader.lines() {
        let line = line.unwrap();
        let skipped = if line.starts_with("addx") {
            let old_value = value;
            value += line.split_whitespace().collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap();
            cycles += 2;
            vec![(cycles - 1, old_value), (cycles, value)]
        } else {
            cycles += 1;
            vec![(cycles, value)]
        };

        for (c, v) in skipped.iter() {
            if (c - 20) % 40 == 0 {
                sum += c * v;
            }
        }
    }

    println!("{}", sum);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut value = 1;
    let mut cycles = 0;
    print!("#");
    for line in reader.lines() {
        let line = line.unwrap();
        let skipped = if line.starts_with("addx") {
            let old_value = value;
            value += line.split_whitespace().collect::<Vec<_>>()[1]
                .parse::<i32>()
                .unwrap();
            cycles += 2;
            vec![(cycles - 1, old_value), (cycles, value)]
        } else {
            cycles += 1;
            vec![(cycles, value)]
        };

        for (c, v) in skipped.iter() {
            let h = c % 40;
            if h == 0 {
                println!();
            }
            if (h-v).abs() < 2 {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}
