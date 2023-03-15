use std::collections::HashMap;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn fibonacci(n: i32, hash: &mut HashMap<i32, (i32, i32)>) -> (i32, i32) {
    if let Some(res) = hash.get(&n) {
        return (res.0, res.1);
    }

    if n == 0 {
        hash.insert(n, (1, 0));
        return (1, 0);
    } else if n == 1 {
        hash.insert(n, (0, 1));
        return (0, 1);
    } else {
        let (zero_1, one_1) = fibonacci(n - 1, hash);
        let (zero_2, one_2) = fibonacci(n - 2, hash);
        hash.insert(n, (zero_1 + zero_2, one_1 + one_2));
        return (zero_1 + zero_2, one_1 + one_2);
    }
}
fn main() {
    let n = read_line_as_number();

    let mut hash = HashMap::new();
    for _ in 0..n {
        let m = read_line_as_number();
        let (zero, one) = fibonacci(m, &mut hash);
        println!("{} {}", zero, one);
    }
}
