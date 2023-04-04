use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read_line_as_number() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        let mut min_heap = BinaryHeap::new();
        let mut max_heap = BinaryHeap::new();
        let mut hash_map = HashMap::new();

        let count = read_line_as_number();
        for _ in 0..count {
            let line = read_line_as_strings();
            match (line[0].as_str(), &line[1]) {
                ("I", n) => {
                    let n = n.parse::<i32>().unwrap();
                    min_heap.push(Reverse(n));
                    max_heap.push(n);
                    *hash_map.entry(n).or_insert(0) += 1;
                }, 
                ("D", n) => {
                    let n = n.parse::<i32>().unwrap();
                    if n == 1 {
                        while let Some(n) = max_heap.pop() {
                            if let Some(count) = hash_map.get_mut(&n) {
                                if *count > 0 {
                                    *count -= 1;
                                    break;
                                }
                            }
                        }
                    } else if n == -1 {
                        while let Some(Reverse(n)) = min_heap.pop() {
                            if let Some(count) = hash_map.get_mut(&n) {
                                if *count > 0 {
                                    *count -= 1;
                                    break;
                                }
                            }
                        }
                    }
                },
                _ => panic!("Invalid input"),
            }
        }

        let max_result = {
            let mut result = None;
            while let Some(n) = max_heap.pop() {
                if let Some(count) = hash_map.get_mut(&n) {
                    if *count > 0 {
                        result = Some(n);
                        break;
                    }
                }
            }
            result
        };

        let min_result = {
            let mut result = None;
            while let Some(Reverse(n)) = min_heap.pop() {
                if let Some(count) = hash_map.get_mut(&n) {
                    if *count > 0 {
                        result = Some(n);
                        break;
                    }
                }
            }
            result
        };

        match (max_result, min_result) {
            (Some(max), Some(min)) => println!("{} {}", max, min),
            _ => println!("EMPTY"),
        }
    }
}
