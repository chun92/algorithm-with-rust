fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut numbers = read_line_as_numbers();
    let target = read_line_as_numbers()[0];

    numbers.sort_unstable();

    let mut count = 0;
    let mut iter1 = numbers.iter();
    let mut iter2 = numbers.iter().rev();

    let mut num1 = iter1.next();
    let mut num2 = iter2.next();

    loop {
        if num1.is_none() || num2.is_none() {
            break;
        }
        let n1 = *num1.unwrap();
        let n2 = *num2.unwrap();
        if n1 >= n2 {
            break;
        }
        let sum = n1 + n2;
        if sum == target {
            count += 1;
            num1 = iter1.next();
        } else if sum >= target {
            num2 = iter2.next();
        } else {
            num1 = iter1.next();
        }
    }

    println!("{}", count);
}
