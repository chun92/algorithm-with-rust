fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut numbers = read_line_as_numbers();
    numbers.sort_unstable();

    let mut iter1 = numbers.iter().peekable();
    let mut iter2 = numbers.iter().rev().peekable();
    let mut num1 = iter1.next();
    let mut num2 = iter2.next();

    let (mut a, mut b) = (*num1.unwrap(), *num2.unwrap());
    let mut result = (a + b).abs();

    while num1.is_some() && num2.is_some() {
        let n1 = *num1.unwrap();
        let n2 = *num2.unwrap();

        if n1 >= n2 {
            break;
        }

        let diff = (n1 + n2).abs();
        if diff < result {
            result = diff;
            (a, b) = (n1, n2);
        }

        let next1 = iter1.peek();
        let next2 = iter2.peek();

        if next1.is_some() && next2.is_some() {
            let next1 = *next1.unwrap();
            let next2 = *next2.unwrap();

            if next1 > next2 {
                break;
            }

            if next1 == next2 {
                let diff1 = (n1 + next2).abs();
                let diff2 = (next1 + n2).abs();
                let min = diff1.min(diff2);
                if min == diff1 {
                    num2 = iter2.next();
                } else {
                    num1 = iter1.next();
                }
            } else {
                let diff1 = (n1 + next2).abs();
                let diff2 = (next1 + n2).abs();
                let diff3 = (next1 + next2).abs();
                let min = diff1.min(diff2).min(diff3);

                if min == diff1 {
                    num2 = iter2.next();
                } else if min == diff2 {
                    num1 = iter1.next();
                } else {
                    num1 = iter1.next();
                    num2 = iter2.next();
                }
            }
        } else {
            break;
        }
    }

    println!("{} {}", a, b);
}
