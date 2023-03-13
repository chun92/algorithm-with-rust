use std::collections::{VecDeque, BinaryHeap};

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|x| x.parse().unwrap()).collect()
}
fn main() {
    let tests = read_line_as_numbers()[0];
    for _ in 0..tests {
        let args = read_line_as_numbers();
        let (n, m) = (args[0], args[1]);

        let nums = read_line_as_numbers();
        let mut queue = 
            nums
            .iter()
            .enumerate()
            .collect::<VecDeque<_>>();
        let mut heap = 
            nums
            .iter()
            .collect::<BinaryHeap<_>>();


        let mut count = 0;

        for _ in 0..n {
            count += 1;
            let target = heap.pop().unwrap();
            let res = queue.iter().position(|x| x.1 == target).unwrap();
            if queue[res].0 == m as usize {
                println!("{}", count);
                break;
            }
            let mut sliced = queue.drain(..res).collect::<VecDeque<_>>();
            queue.append(&mut sliced);
            queue.pop_front();
        }
    }
}
