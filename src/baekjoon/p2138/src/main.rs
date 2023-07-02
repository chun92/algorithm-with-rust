fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn read_line_as_0_and_1() -> Vec<bool> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    input.chars().map(|c| {
        let x = c.to_digit(10).unwrap() as usize;
        if x == 0 {
            false
        } else {
            true
        }
    }).collect()
}

fn main() {
    let n = read_line_as_number();
    let given = read_line_as_0_and_1();
    let target = read_line_as_0_and_1();

    let mut answer = u64::MAX;
    let mut first = given.clone();
    first[0] = !first[0];
    first[1] = !first[1];
    let mut count = 1;
    let mut prev = first[0];
    for i in 1..n - 1 {
        if prev != target[i - 1] {
            count += 1;
            first[i - 1] = !first[i - 1];
            first[i] = !first[i];
            first[i + 1] = !first[i + 1];
        }
        prev = first[i];
    }
    if first[n - 2] != target[n - 2] {
        count += 1;
        first[n - 2] = !first[n - 2];
        first[n - 1] = !first[n - 1];
    }
    if first[n - 1] == target[n - 1] {
        answer = count;
    }

    let mut second = given;
    let mut count = 0;
    let mut prev = second[0];
    for i in 1..n - 1 {
        if prev != target[i - 1] {
            count += 1;
            second[i - 1] = !second[i - 1];
            second[i] = !second[i];
            second[i + 1] = !second[i + 1];
        }
        prev = second[i];
    }
    if second[n - 2] != target[n - 2] {
        count += 1;
        second[n - 2] = !second[n - 2];
        second[n - 1] = !second[n - 1];
    }
    if second[n - 1] == target[n - 1] {
        answer = std::cmp::min(answer, count);
    }

    if answer == u64::MAX {
        println!("{}", -1);
    } else {
        println!("{}", answer);
    }
}
