use std::collections::HashMap;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn dp(vec: &Vec<i32>, n: i32) -> i32 {
    let mut dp = HashMap::new();
    if n == 0 {
        return vec[0];
    }

    if n == 1 {
        return vec[0] + vec[1];
    }

    dp.insert(0, (vec[0], vec[0]));
    dp.insert(1, (vec[0] + vec[1], vec[1]));

    for i in 2..=n {
        let f1 = vec[i as usize] + dp.get(&(i - 1)).unwrap().1;
        let f2 = vec[i as usize] 
            + std::cmp::max(dp.get(&(i - 2)).unwrap().0, dp.get(&(i - 2)).unwrap().1);
        dp.insert(i, (f1, f2));
    }

    std::cmp::max(dp.get(&n).unwrap().0, dp.get(&n).unwrap().1)
}

fn main() {
    let n = read_line_as_number();
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_number());
    }
    println!("{}", dp(&vec, n - 1));
}
