use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

fn main() {
    part_1();
    part_2();
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum Tree {
    Node(Vec<Tree>),
    Leaf(i32),
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Tree::Node(l), Tree::Node(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    match l.cmp(r) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                l.len().cmp(&r.len())
            }
            (l @ Tree::Node(_), r @ Tree::Leaf(_)) => l.cmp(&Tree::Node(vec![r.clone()])),
            (l @ Tree::Leaf(_), r @ Tree::Node(_)) => Tree::Node(vec![l.clone()]).cmp(r),
            (Tree::Leaf(l), Tree::Leaf(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Tree {
    if *chars.peek().unwrap() == '[' {
        let mut vector = vec![];
        while *chars.peek().unwrap() != ']' {
            chars.next();
            if *chars.peek().unwrap() != ']' {
                vector.push(parse(chars));
            }
        }
        chars.next();

        Tree::Node(vector)
    } else {
        let mut num = vec![];
        while *chars.peek().unwrap() != ']' && *chars.peek().unwrap() != ',' {
            num.push(chars.next().unwrap());
        }
        let num = num.into_iter().collect::<String>().parse::<i32>().unwrap();
        Tree::Leaf(num)
    }
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut index = 1;
    let mut sum = 0;
    while let Some(first) = lines.next() {
        let first = parse(&mut first.unwrap().chars().peekable());
        let second = parse(&mut lines.next().unwrap().unwrap().chars().peekable());
        if first < second {
            sum += index;
        }
        lines.next();
        index += 1;
    }

    println!("{}", sum);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);

    let mut trees: Vec<Tree> = reader
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            if !line.is_empty() {
                Some(parse(&mut line.chars().peekable()))
            } else {
                None
            }
        })
        .collect();

    let divider1 = Tree::Node(vec![Tree::Node(vec![Tree::Leaf(2)])]);
    let divider2 = Tree::Node(vec![Tree::Node(vec![Tree::Leaf(6)])]);
    trees.push(divider1.clone());
    trees.push(divider2.clone());

    trees.sort();

    let pos1 = trees.iter().position(|e| e == &divider1).unwrap();
    let pos2 = trees.iter().position(|e| e == &divider2).unwrap();

    println!("{}", (pos1 + 1) * (pos2 + 1));
}
