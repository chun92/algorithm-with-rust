fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn dp(n: u64, vec: &mut Vec<Vec<u64>>) -> u64 {
    for i in 0..10 {
        vec[0][i] = 1;
    }

    for i in 1..n as usize {
        for j in 0..10 {
            if j == 0 {
                vec[i][j] = vec[i - 1][j + 1] % 1000000000;
            } else if j == 9 {
                vec[i][j] = vec[i - 1][j - 1] % 1000000000;
            } else {
                vec[i][j] = (vec[i - 1][j - 1] + vec[i - 1][j + 1]) % 1000000000;
            }
        }
    }

    let mut sum = 0;
    for i in 1..10 {
        sum = (sum + vec[n as usize - 1][i]) % 1000000000;
    }
    sum % 1000000000
}

fn main() {
    let n = read_line_as_number();
    let mut vec = vec![vec![0; 10]; n as usize];
    println!("{}", dp(n, &mut vec) % 1000000000);
}
