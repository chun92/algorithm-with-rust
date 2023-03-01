fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_prime_impl(num: usize) -> bool {
    if num == 1 {
        return false;
    }
    let limit = (num as f64).sqrt().floor() as usize;
    let mut result = true;
    for i in 2..=limit {
        if num % i == 0 {
            result = false;
        }
    }
    result
}

fn is_prime(table: &mut Vec<bool>, current_check: &mut usize, num: usize) -> bool {
    if *current_check >= num {
        table[num]
    } else {
        for i in (*current_check + 1)..num {
            table[i] = is_prime_impl(i);
        }
        is_prime_impl(num)
    }
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();

    let mut memoization_table = vec![false; 1001];
    let mut current_check = 0;
    
    let result = numbers
        .into_iter()
        .filter(|n| is_prime(&mut memoization_table, &mut current_check, *n as usize))
        .collect::<Vec<i32>>()
        .len();

    println!("{}", result);
}
