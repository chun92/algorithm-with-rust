fn read_line_as_numbers() -> Vec<u16> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_binomial_coefficient(vec: &mut Vec<Vec<u64>>, n: u16, k: u16) -> u64 {
    if k == 0 || n == k {
        vec[n as usize][std::cmp::min(k, n - k) as usize] = 1;
        return 1;
    }

    if vec[n as usize][std::cmp::min(k, n - k) as usize] != 0 {
        return vec[n as usize][std::cmp::min(k, n - k) as usize];
    }

    let result = (get_binomial_coefficient(vec, n - 1, k - 1) + get_binomial_coefficient(vec, n - 1, k)) % 10007;
    vec[n as usize][std::cmp::min(k, n - k) as usize] = result;
    return result;
}

fn main() {
    let args = read_line_as_numbers();
    let (n, k) = (args[0], args[1]);
    let mut vec: Vec<Vec<u64>> = vec![vec![0; 1001]; 1001];
    println!("{}", get_binomial_coefficient(&mut vec, n, k));
}
