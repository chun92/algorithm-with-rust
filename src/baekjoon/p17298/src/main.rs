fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: usize, numbers: &mut Vec<i32>) {
    let mut stack = Vec::new();
    for i in 0..n {
        while let Some(&(index, number)) = stack.last() {
            if number < numbers[i] {
                stack.pop();
                numbers[index] = numbers[i];
            } else {
                break;
            }
        }
        stack.push((i, numbers[i]));
    }

    while let Some((index, _)) = stack.pop() {
        numbers[index] = -1;
    }
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut numbers = read_line_as_numbers();

    solve(n as usize, &mut numbers);
    let result =
        numbers.iter().map(|number| number.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", result);
}
