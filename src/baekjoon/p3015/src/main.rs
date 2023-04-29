use std::io::{stdin, StdinLock, BufRead};


fn read_line_as_number(stdin: &mut StdinLock) -> u64 {
    let mut read = String::new();
    stdin.read_line(&mut read).unwrap();
    read.trim().parse::<u64>().unwrap()
}



fn main() {
    let mut stdin = stdin().lock();

    let n = read_line_as_number(&mut stdin);
    let mut stack = Vec::new();
    let mut result: u64 = 0;
    for _ in 0..n {
        let x = read_line_as_number(&mut stdin);
        let mut same_count = 1;
        while let Some(&(number, count)) = stack.last() {
            if number < x {
                result += count;
                stack.pop();
            } else if number == x {
                same_count += count;
                result += count;
                stack.pop();
            } else {
                result += 1;
                break;
            }
        }
        stack.push((x, same_count));
    }
    println!("{}", result);
}
