use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

struct MyTuple(i32, i32);

impl Ord for MyTuple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Equal => self.1.cmp(&other.1),
            ord => ord,
        }
    }
}

impl Eq for MyTuple {}

impl PartialOrd for MyTuple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MyTuple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn main() {
    let n = read_line_as_number();
    
    let mut heap = BinaryHeap::<Reverse<MyTuple>>::new();
    
    for _ in 0..n {
        let num = read_line_as_number();
        if num == 0 {
            if heap.is_empty() {
                println!("0");
            } else {
                println!("{}", heap.pop().unwrap().0.1);
            }
        } else {
            heap.push(Reverse(MyTuple(num.abs(), num)));
        }
    }
}
