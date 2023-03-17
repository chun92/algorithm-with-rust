use std::collections::HashMap;
use std::io::{BufWriter, Write, stdin, stdout};

fn read_line_as_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn read_line_as_strings() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}

fn get_sum(map: &HashMap<char, Vec<usize>>, c: char, index: usize) -> usize {
    let vec = map.get(&c).unwrap();
    let mut result = 0;
    
    for (_, v) in vec.iter().enumerate() {
        if *v <= index {
            result += 1;
        } else {
            break;
        }
    }
    result
}

fn get_partial_sum(map: &HashMap<char, Vec<usize>>, c: char, start: usize, end: usize) -> usize {
    let big = get_sum(map, c, end);
    if start == 0 {
        return big;
    }
    let small = get_sum(map, c, start - 1);
    big - small
}

fn main() {
    let target = read_line_as_string().trim().to_string();
    let mut map = HashMap::new();
    ('a'..='z').for_each(|c| {
        map.insert(c, Vec::new());
    });

    target
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            map.get_mut(&c).unwrap().push(i);
        });
    
    let n = read_line_as_string().trim().parse::<usize>().unwrap();

    let mut out = BufWriter::new(stdout());
    for _ in 0..n {
        let line = read_line_as_strings();
        let c = line[0].chars().next().unwrap();
        let start = line[1].parse::<usize>().unwrap();
        let end = line[2].parse::<usize>().unwrap();
        writeln!(out, "{}", get_partial_sum(&map, c, start, end)).unwrap();
    }
}
