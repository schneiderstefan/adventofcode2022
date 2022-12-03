use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashSet,
};

fn main() {
    part_1();
    part_2();
}

fn common(line: &str) -> char {
    let len = line.len();
    let first = &line[0..len/2];
    let second = &line[len/2..];
    assert_eq!(first.len(), second.len());

    let mut chars: HashSet<char> = Default::default();
    for c in first.chars() {
        chars.insert(c);
    }

    for c in second.chars() {
        if chars.contains(&c) {
            return c
        }
    }
    unreachable!();
}

fn union(lines: &[String]) -> char {
    let mut overlap: HashSet<char> = lines[0].chars().collect();

    for line in &lines[1..] {
        let mut new_overlap: HashSet<char> = Default::default();
        for c in line.chars() {
            if overlap.contains(&c) {
                new_overlap.insert(c);
            }
        }
        overlap = new_overlap;
    }

    assert_eq!(overlap.len(), 1);
    *overlap.iter().next().unwrap()
}

fn priority(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        return c as u32 - 'a' as u32 + 1;
    } else if  ('A'..='Z').contains(&c) {
        return c as u32 - 'A' as u32 + 27;
    }

    unreachable!();
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut points = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let common = common(&line);
        points += priority(common);
    }

    println!("{:?}", points);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut points = 0;
    let mut group: Vec<String> = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        group.push(line);

        if group.len() == 3 {
            let common = union(&group);
            points += priority(common);
            group = Default::default();
        }
    }

    println!("{:?}", points);

}
