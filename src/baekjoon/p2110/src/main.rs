fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_number() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn check_distance(numbers: &Vec<u32>, mid: u32, target: u32) -> bool {
    let mut count = 1;
    let mut prev = *numbers.first().unwrap();

    for number in numbers {
        if number - prev >= mid {
            count += 1;

            prev = *number;
        }
    }

    count >= target
}

fn solve(numbers: &Vec<u32>, min: u32, max: u32, target: u32) -> u32 {
    if min >= max {
        return min;
    }

    let mid = (min + max) / 2;

    let result = check_distance(numbers, mid, target);

    if result {
        if mid == min {
            if check_distance(numbers, mid + 1, target) {
                return mid + 1;
            } else {
                return mid;
            }
        } else {
            solve(numbers, mid, max, target)
        }
    } else {
        solve(numbers, min, mid - 1, target)
    }
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut numbers = Vec::new();

    for _ in 0..n {
        let number = read_line_as_number();
        numbers.push(number);
    }

    numbers.sort();

    let min = 0;
    let max = numbers.last().unwrap() - numbers.first().unwrap();
    let result = solve(&numbers, min, max, m);
    println!("{}", result);
}
