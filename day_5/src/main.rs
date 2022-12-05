use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() {
    part_1();
    part_2();
}

fn parse_stack_line(line: String) -> Vec<char> {
    let mut chars = line.chars();
    let mut result: Vec<char> = Default::default();
    while let Some(_) = chars.next() {
        result.push(chars.next().unwrap());
        chars.next();
        chars.next();
    }

    result
}

fn parse_stacks<T>(input: &mut T) -> Vec<VecDeque<char>>
where
    T: Iterator<Item = Result<String, Error>>,
{
    let mut lines: Vec<Vec<char>> = Default::default();

    for line in input {
        let line = line.unwrap();
        if line.starts_with(" 1") {
            break;
        }
        lines.push(parse_stack_line(line));
    }
    let mut result = vec![VecDeque::<char>::default(); lines[0].len()];
    for line in lines {
        for (i, c) in line.iter().enumerate() {
            if *c != ' ' {
                result[i].push_front(*c)
            }
        }
    }
    result
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut stacks = parse_stacks(&mut lines);
    lines.next();
    for line in lines {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let count: usize = words[1].parse().unwrap();
        let from: usize = words[3].parse().unwrap();
        let to: usize = words[5].parse().unwrap();
        for _ in 0..count {
            let item = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(item);
        }
    }

    let result: String = stacks.iter().map(|stack| *stack.back().unwrap()).collect();

    println!("{}", result);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut stacks = parse_stacks(&mut lines);
    lines.next();
    for line in lines {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        let count: usize = words[1].parse().unwrap();
        let from: usize = words[3].parse().unwrap();
        let to: usize = words[5].parse().unwrap();
        let mut substack: Vec<char> = Default::default();
        for _ in 0..count {
            let item = stacks[from - 1].pop_back().unwrap();
            substack.push(item);
        }
        stacks[to - 1].extend(substack.iter().rev())
    }

    let result: String = stacks.iter().map(|stack| *stack.back().unwrap()).collect();

    println!("{}", result);
}
