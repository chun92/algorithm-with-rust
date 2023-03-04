fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn get_house_number(k: usize, n: usize) -> usize {
    let mut house: Vec<Vec<usize>> = (0..=k).map(|_| vec![0; n + 1]).collect();
    house[0] = (1..=n).collect();

    for i in 1..=k {
        for j in 1..=n {
            house[i][j - 1] = {
                let mut sum = 0;
                    for b in 1..=j {
                        sum += house[i - 1][b - 1]
                    }
                sum
            };
        }
    }

    house[k][n - 1]
}

fn main() {
    let count = read_line_as_number();
    for _ in 0..count {
        let k = read_line_as_number();
        let n = read_line_as_number();
        println!("{}", get_house_number(k, n));       
    }
}
