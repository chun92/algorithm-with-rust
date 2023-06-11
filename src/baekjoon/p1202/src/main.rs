use std::collections::BinaryHeap;

fn main() {
    let (n, k) = {
        let input = read_line_as_numbers();
        (input[0] as usize, input[1] as usize)
    };

    let mut jewels: Vec<(u64, u64)> = Vec::new();
    for _ in 0..n {
        let input = read_line_as_numbers();
        jewels.push((input[0], input[1]));
    }

    let mut bags: Vec<u64> = Vec::new();
    for _ in 0..k {
        let input = read_line_as_numbers();
        bags.push(input[0]);
    }

    jewels.sort();
    bags.sort();

    let mut max_heap: BinaryHeap<u64> = BinaryHeap::new();
    let mut max_value: u64 = 0;
    let mut j = 0;

    for i in 0..k {
        while j < n && jewels[j].0 <= bags[i] {
            max_heap.push(jewels[j].1);
            j += 1;
        }
        if !max_heap.is_empty() {
            max_value += max_heap.pop().unwrap();
        }
    }

    println!("{}", max_value);
}

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
