use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Op {
    Plus(i64),
    Mult(i64),
    Square,
    Double,
}

impl Op {
    fn execute(&self, input: i64) -> i64 {
        match self {
            Op::Plus(x) => input + x,
            Op::Mult(x) => input * x,
            Op::Square => input * input,
            Op::Double => input * 2,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Op,
    test: i64,
    iftrue: usize,
    iffalse: usize,
}

fn parse_input() -> Vec<Monkey> {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut result: Vec<Monkey> = Default::default();
    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        if line.unwrap().is_empty() {
            lines.next();
        }
        let items = lines.next().unwrap().unwrap();
        let items: Vec<&str> = items.split(": ").collect();
        let items: Vec<i64> = items[1].split(", ").map(|x| x.parse().unwrap()).collect();

        let operation = lines.next().unwrap().unwrap();
        let operation: Vec<&str> = operation.split_whitespace().collect();
        assert_eq!(operation[0], "Operation:");
        assert_eq!(operation[1], "new");
        assert_eq!(operation[2], "=");
        assert_eq!(operation[3], "old");
        let operation = match operation[4] {
            "+" => match operation[5] {
                "old" => Op::Double,
                x => Op::Plus(x.parse().unwrap()),
            },
            "*" => match operation[5] {
                "old" => Op::Square,
                x => Op::Mult(x.parse().unwrap()),
            },
            _ => unreachable!(),
        };

        let test = lines.next().unwrap().unwrap();
        let test: Vec<&str> = test.split_whitespace().collect();
        assert_eq!(test[0], "Test:");
        let test = test[3].parse().unwrap();

        let iftrue = lines.next().unwrap().unwrap();
        let iftrue: Vec<&str> = iftrue.split_whitespace().collect();
        assert_eq!(iftrue[0], "If");
        assert_eq!(iftrue[1], "true:");
        let iftrue = iftrue[5].parse().unwrap();

        let iffalse = lines.next().unwrap().unwrap();
        let iffalse: Vec<&str> = iffalse.split_whitespace().collect();
        assert_eq!(iffalse[0], "If");
        assert_eq!(iffalse[1], "false:");
        let iffalse = iffalse[5].parse().unwrap();

        let monkey = Monkey {
            items,
            operation,
            test,
            iftrue,
            iffalse,
        };
        result.push(monkey);
    }

    result
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut monkeys = parse_input();
    let mut counts = vec![0; monkeys.len()];

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[m].items);
            for item in items {
                let item = monkeys[m].operation.execute(item);
                let item = item / 3;
                let target = if item % monkeys[m].test == 0 {
                    monkeys[m].iftrue
                } else {
                    monkeys[m].iffalse
                };
                monkeys[target].items.push(item);
                counts[m] += 1;
            }
        }
    }

    counts.sort_by_key(|x| Reverse(*x));
    println!("{:?}", counts[0] * counts[1]);
}

fn part_2() {
    let mut monkeys = parse_input();
    let mut counts = vec![0_i64; monkeys.len()];
    let modulo: i64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[m].items);
            for item in items {
                let item = monkeys[m].operation.execute(item);
                let item = item % modulo;
                let target = if item % monkeys[m].test == 0 {
                    monkeys[m].iftrue
                } else {
                    monkeys[m].iffalse
                };
                monkeys[target].items.push(item);
                counts[m] += 1;
            }
        }
    }

    counts.sort_by_key(|x| Reverse(*x));
    println!("{:?}", counts[0] * counts[1]);
}
