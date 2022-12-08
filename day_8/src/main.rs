use ndarray::{Array2, Zip};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_1();
    part_2();
}

fn process_visibility<'a, D, V>(data: D, visible: V)
where
    D: Iterator<Item = &'a u32>,
    V: Iterator<Item = &'a mut bool>,
{
    let mut max = -1;
    for (elem, v) in data.zip(visible) {
        let elem = *elem as i32;
        if elem > max {
            *v = true;
            max = elem;
        }
    }
}

fn process_range<'a, D, V>(data: D, score: V)
where
    D: Iterator<Item = &'a u32>,
    V: Iterator<Item = &'a mut u32>,
{
    let mut stack: Vec<(usize, u32)> = Default::default();

    for (i, (elem, s)) in data.zip(score).enumerate() {
        while !stack.is_empty() && stack.last().unwrap().1 < *elem {
            stack.pop();
        }

        *s *= if stack.is_empty() {
            i as u32
        } else {
            (i - stack.last().unwrap().0) as u32
        };

        if !stack.is_empty() && stack.last().unwrap().1 == *elem {
            stack.pop();
        }
        stack.push((i, *elem));
    }
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let data: Vec<u32> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .map(|i| i.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let height = reader.lines().count();

    let shape = (height, data.len() / height);

    let data: Array2<u32> = Array2::from_shape_vec(shape, data).unwrap();
    let mut visible = Array2::from_elem(shape, false);

    Zip::from(data.columns())
        .and(visible.columns_mut())
        .for_each(|row, mut visible| {
            process_visibility(row.iter(), visible.iter_mut());
            process_visibility(row.iter().rev(), visible.iter_mut().rev());
        });

    Zip::from(data.rows())
        .and(visible.rows_mut())
        .for_each(|row, mut visible| {
            process_visibility(row.iter(), visible.iter_mut());
            process_visibility(row.iter().rev(), visible.iter_mut().rev());
        });

    let sum: u32 = visible.iter().map(|i| if *i { 1 } else { 0 }).sum();

    println!("{}", sum);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let data: Vec<u32> = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .map(|i| i.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let height = reader.lines().count();

    let shape = (height, data.len() / height);

    let data: Array2<u32> = Array2::from_shape_vec(shape, data).unwrap();
    let mut score = Array2::from_elem(shape, 1);

    Zip::from(data.columns())
        .and(score.columns_mut())
        .for_each(|row, mut visible| {
            process_range(row.iter(), visible.iter_mut());
            process_range(row.iter().rev(), visible.iter_mut().rev());
        });

    Zip::from(data.rows())
        .and(score.rows_mut())
        .for_each(|row, mut visible| {
            process_range(row.iter(), visible.iter_mut());
            process_range(row.iter().rev(), visible.iter_mut().rev());
        });

    println!("{}", score.iter().max().unwrap());
}
