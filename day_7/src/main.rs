use std::{
    collections::HashMap,
    fs::File,
    io,
    io::{BufRead, BufReader},
    iter::Peekable,
    path::PathBuf,
};

fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
struct Dir {
    subdirs: Vec<String>,
    files: Vec<(String, usize)>,
    size: Option<usize>,
}

fn cd(pwd: &mut PathBuf, dir: &str) {
    match dir {
        "/" => *pwd = PathBuf::from("/"),
        ".." => {
            pwd.pop();
        }
        dir => pwd.push(dir),
    }
}

fn ls<T>(lines: &mut Peekable<T>) -> Dir
where
    T: Iterator<Item = io::Result<String>>,
{
    let mut subdirs: Vec<String> = Default::default();
    let mut files: Vec<(String, usize)> = Default::default();
    while let Some(line) = lines.next_if(|line| !line.as_ref().unwrap().starts_with('$')) {
        let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();
        match split[0] {
            "dir" => subdirs.push(split[1].to_string()),
            size => files.push((split[1].to_string(), size.parse().unwrap())),
        }
    }

    Dir {
        subdirs,
        files,
        size: None,
    }
}

fn recursive_sizes(dirs: &mut HashMap<PathBuf, Dir>, root: PathBuf) {
    let mut dir = dirs.remove(&root).unwrap();
    let mut sum = 0;
    for subdir in &dir.subdirs {
        let mut path = root.clone();
        path.push(subdir);
        recursive_sizes(dirs, path.clone());
        sum += dirs.get(&path).unwrap().size.unwrap();
    }
    sum += dir.files.iter().map(|(_, size)| size).sum::<usize>();

    dir.size = Some(sum);
    dirs.insert(root, dir);
}

fn part_1() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut pwd = PathBuf::from("/");
    let mut dirs: HashMap<PathBuf, Dir> = Default::default();
    let mut lines = reader.lines().peekable();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();
        if let "$" = split[0] {
            match split[1] {
                "ls" => {
                    if !dirs.contains_key(&pwd) {
                        let dir = ls(&mut lines);
                        dirs.insert(pwd.clone(), dir);
                    }
                }
                "cd" => cd(&mut pwd, split[2]),
                _ => unreachable!(),
            }
        }
    }

    recursive_sizes(&mut dirs, PathBuf::from("/"));

    let mut sum = 0;
    for dir in dirs.values() {
        if dir.size.unwrap() <= 100000 {
            sum += dir.size.unwrap();
        }
    }
    println!("{}", sum);
}

fn part_2() {
    let file = File::open("src/input").unwrap();
    let reader = BufReader::new(file);
    let mut pwd = PathBuf::from("/");
    let mut dirs: HashMap<PathBuf, Dir> = Default::default();
    let mut lines = reader.lines().peekable();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();
        if let "$" = split[0] {
            match split[1] {
                "ls" => {
                    if !dirs.contains_key(&pwd) {
                        let dir = ls(&mut lines);
                        dirs.insert(pwd.clone(), dir);
                    }
                }
                "cd" => cd(&mut pwd, split[2]),
                _ => unreachable!(),
            }
        }
    }

    recursive_sizes(&mut dirs, PathBuf::from("/"));

    let total = dirs.get(&PathBuf::from("/")).unwrap().size.unwrap();
    let to_delete = total - 40000000;
    let mut min = total;
    for dir in dirs.values() {
        let size = dir.size.unwrap();
        if size >= to_delete {
            min = min.min(size);
        }
    }

    println!("{}", min);
}
