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
        let ranges: Vec<_> = line
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].len(), 2);
        assert_eq!(ranges[1].len(), 2);

        if (ranges[0][0] <= ranges[1][0] && ranges[0][1] >= ranges[1][1])
            || (ranges[1][0] <= ranges[0][0] && ranges[1][1] >= ranges[0][1])
        {
            points += 1;
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
        let ranges: Vec<_> = line
            .split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].len(), 2);
        assert_eq!(ranges[1].len(), 2);

        let (left, right) = if ranges[0][0] <= ranges[1][0] {
            (&ranges[0], &ranges[1])
        } else {
            (&ranges[1], &ranges[0])
        };

        if left[1] >= right[0] {
            points+=1;
        }
    }

    println!("{:?}", points);
}
