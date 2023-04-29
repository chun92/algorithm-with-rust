use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: usize, numbers: &mut Vec<i32>, counts: &HashMap<&i32, i32>) {
    let mut stack = Vec::new();
    for i in 0..n {
        let cur_count = *counts.get(&numbers[i]).unwrap();
        while let Some(&(index, number)) = stack.last() {
            if number < cur_count {
                stack.pop();
                numbers[index] = numbers[i];
            } else {
                break;
            }
        }
        stack.push((i, cur_count));
    }

    while let Some((index, _)) = stack.pop() {
        numbers[index] = -1;
    }
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut numbers = read_line_as_numbers();
    let numbers_clone = numbers.clone();
    let mut counts = HashMap::new();
    numbers_clone
        .iter()
        .for_each(
            |number| *counts.entry(number).or_insert(0) += 1,
        );

    solve(n as usize, &mut numbers, &counts);
    let result =
        numbers.iter().map(|number| number.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}
