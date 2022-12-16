use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part_1();
    part_2();
}

fn insert_if_bigger(
    situations: &mut BTreeMap<BTreeSet<usize>, usize>,
    situation: BTreeSet<usize>,
    value: usize,
) {
    for (sit, val) in situations.iter() {
        if *val > value && sit.is_superset(&situation) {
            return;
        }
    }
    situations.insert(situation, value);
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let graph: HashMap<String, (usize, Vec<String>)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<_> = line.split_whitespace().collect();
            let name = split[1].to_string();
            let rate = split[4]
                .trim_start_matches("rate=")
                .trim_end_matches(';')
                .parse()
                .unwrap();
            let neighbours = split[9..]
                .iter()
                .map(|s| s.trim_end_matches(',').to_string())
                .collect();

            (name, (rate, neighbours))
        })
        .collect();

    let indices: HashMap<String, usize> = graph
        .keys()
        .enumerate()
        .map(|(i, key)| (key.clone(), i))
        .collect();
    let graph: HashMap<usize, (usize, Vec<usize>)> = graph
        .iter()
        .map(|(k, v)| {
            (
                indices[k],
                (v.0, v.1.iter().map(|s| indices[s]).collect::<Vec<_>>()),
            )
        })
        .collect();

    let mut situations: Vec<BTreeMap<BTreeSet<usize>, usize>> =
        vec![Default::default(); graph.len()];
    situations[indices["AA"]].insert(Default::default(), 0);

    for i in 0..30 {
        let mut next_situations: Vec<BTreeMap<BTreeSet<usize>, usize>> =
            vec![Default::default(); graph.len()];
        for (position, situations) in situations.iter().enumerate() {
            for (situation, value) in situations {
                let rate = graph[&position].0;
                if rate > 0 && !situation.contains(&position) {
                    let mut new_situation = situation.clone();
                    new_situation.insert(position);
                    let new_value = value + (30 - i - 1) * rate;
                    insert_if_bigger(&mut next_situations[position], new_situation, new_value);
                }
                for neighbour in &graph[&position].1 {
                    insert_if_bigger(&mut next_situations[*neighbour], situation.clone(), *value);
                }
            }
        }

        situations = next_situations;
    }

    println!(
        "{}",
        situations
            .iter()
            .map(|set| set.values().max().unwrap())
            .max()
            .unwrap()
    );
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let graph: HashMap<String, (usize, Vec<String>)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let split: Vec<_> = line.split_whitespace().collect();
            let name = split[1].to_string();
            let rate = split[4]
                .trim_start_matches("rate=")
                .trim_end_matches(';')
                .parse()
                .unwrap();
            let neighbours = split[9..]
                .iter()
                .map(|s| s.trim_end_matches(',').to_string())
                .collect();

            (name, (rate, neighbours))
        })
        .collect();

    let indices: HashMap<String, usize> = graph
        .keys()
        .enumerate()
        .map(|(i, key)| (key.clone(), i))
        .collect();
    let graph: HashMap<usize, (usize, Vec<usize>)> = graph
        .iter()
        .map(|(k, v)| {
            (
                indices[k],
                (v.0, v.1.iter().map(|s| indices[s]).collect::<Vec<_>>()),
            )
        })
        .collect();

    let max_rate: usize = graph.values().map(|v| v.0).sum();

    let mut situations: Vec<BTreeMap<BTreeSet<usize>, usize>> =
        vec![Default::default(); graph.len() * graph.len()];
    situations[indices["AA"] * graph.len() + indices["AA"]].insert(Default::default(), 0);
    let mut visited: Vec<BTreeSet<BTreeSet<usize>>> =
        vec![Default::default(); graph.len() * graph.len()];

    let mut max = 0;
    let duration = 26;

    for i in 0..duration {
        // println!(
        //     "{}: {}",
        //     i,
        //     situations.iter().map(|set| set.len()).sum::<usize>()
        // );

        let mut next_situations: Vec<BTreeMap<BTreeSet<usize>, usize>> =
            vec![Default::default(); graph.len() * graph.len()];
        let mut new_visited: Vec<BTreeSet<BTreeSet<usize>>> =
            vec![Default::default(); graph.len() * graph.len()];
        for (position, situations) in situations.iter().enumerate() {
            let pos1 = position / graph.len();
            let pos2 = position % graph.len();
            for (situation, value) in situations {
                let max_possible: usize = value
                    + (duration - i - 1)
                        * (max_rate - situation.iter().map(|pos| graph[pos].0).sum::<usize>());
                if max_possible >= max {
                    if pos1 != pos2
                        && graph[&pos1].0 > 0
                        && graph[&pos2].0 > 0
                        && !situation.contains(&pos1)
                        && !situation.contains(&pos2)
                    {
                        let mut new_situation = situation.clone();
                        new_situation.insert(pos1);
                        new_situation.insert(pos2);
                        let new_value = value + (duration - i - 1) * (graph[&pos1].0 + graph[&pos2].0);
                        if !visited[position].contains(&new_situation) {
                            new_visited[position].insert(new_situation.clone());
                            max = max.max(new_value);
                            insert_if_bigger(
                                &mut next_situations[position],
                                new_situation,
                                new_value,
                            );
                        }
                    }

                    if graph[&pos1].0 > 0 && !situation.contains(&pos1) {
                        for neighbour2 in &graph[&pos2].1 {
                            let mut new_situation = situation.clone();
                            new_situation.insert(pos1);
                            let new_value = value + (duration - i - 1) * graph[&pos1].0;
                            let new_position = pos1 * graph.len() + neighbour2;
                            if !visited[new_position].contains(&new_situation) {
                                new_visited[new_position].insert(new_situation.clone());
                                max = max.max(new_value);
                                insert_if_bigger(
                                    &mut next_situations[new_position],
                                    new_situation,
                                    new_value,
                                );
                            }
                        }
                    }

                    if graph[&pos2].0 > 0 && !situation.contains(&pos2) {
                        for neighbour1 in &graph[&pos1].1 {
                            let mut new_situation = situation.clone();
                            new_situation.insert(pos2);
                            let new_value = value + (duration - i - 1) * graph[&pos2].0;
                            let new_position = neighbour1 * graph.len() + pos2;
                            if !visited[new_position].contains(&new_situation) {
                                new_visited[new_position].insert(new_situation.clone());
                                max = max.max(new_value);
                                insert_if_bigger(
                                    &mut next_situations[new_position],
                                    new_situation,
                                    new_value,
                                );
                            }
                        }
                    }

                    for neighbour1 in &graph[&pos1].1 {
                        for neighbour2 in &graph[&pos2].1 {
                            let new_position = neighbour1 * graph.len() + neighbour2;
                            if !visited[new_position].contains(situation) {
                                new_visited[new_position].insert(situation.clone());
                                insert_if_bigger(
                                    &mut next_situations[new_position],
                                    situation.clone(),
                                    *value,
                                );
                            }
                        }
                    }
                }
            }
        }

        for (visited, new) in visited.iter_mut().zip(new_visited) {
            visited.extend(new.into_iter());
        }
        situations = next_situations;
    }

    println!(
        "{}",
        situations
            .into_iter()
            .map(|set| set.into_values().max().unwrap_or_default())
            .max()
            .unwrap()
    );
}
