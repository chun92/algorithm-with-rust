fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (_n, s) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let numbers = read_line_as_numbers();

    if numbers[0] >= s {
        println!("1");
        return;
    }

    let mut iter1 = numbers.iter();
    let mut iter2 = numbers.iter().skip(1);

    let mut num1 = iter1.next();
    let mut num2 = iter2.next();
    let mut sum = num1.unwrap() + num2.unwrap();
    let mut result = usize::MAX;

    loop {
        if iter1.len() < iter2.len() {
            break;
        }
        
        if num1.is_none() || num2.is_none() {
            break;
        }

        if sum >= s {
            result = result.min(iter1.len() - iter2.len() + 1);
            sum -= num1.unwrap();
            num1 = iter1.next();
        } else {
            num2 = iter2.next();
            if let Some(n) = num2 {
                sum += n;
            }
        }
    }

    if result == usize::MAX {
        println!("0");
    } else {
        println!("{}", result);
    }
}