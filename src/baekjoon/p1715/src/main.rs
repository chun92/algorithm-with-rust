use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read_line_as_number() -> u64 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}

fn main() {
    let n = read_line_as_number();
    let mut heap = BinaryHeap::new();

    for _ in 0..n {
        let x = read_line_as_number();
        heap.push(Reverse(x));
    }

    let mut sum = 0;
    while heap.len() > 1 {
        let a = heap.pop().unwrap().0;
        let b = heap.pop().unwrap().0;
        let c = a + b;
        sum += c;
        heap.push(Reverse(c));
    }

    println!("{}", sum);
}
