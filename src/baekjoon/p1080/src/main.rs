fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_0_and_1s() -> Vec<bool> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut given = vec![vec![false; m]; n];
    for i in 0..n {
        let v = read_line_as_0_and_1s();
        for j in 0..m {
            given[i][j] = v[j];
        }
    }

    let mut answer = vec![vec![false; m]; n];
    for i in 0..n {
        let v = read_line_as_0_and_1s();
        for j in 0..m {
            answer[i][j] = v[j];
        }
    }

    if n < 3 || m < 3 {
        for i in 0..n {
            for j in 0..m {
                if given[i][j] != answer[i][j] {
                    println!("-1");
                    return;
                }
            }
        }
        println!("0");
        return;
    }

    let mut count = 0;
    for i in 0..n - 2 {
        for j in 0..m - 2 {
            if given[i][j] != answer[i][j] {
                count += 1;
                for k in 0..3 {
                    for l in 0..3 {
                        given[i + k][j + l] = !given[i + k][j + l];
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if given[i][j] != answer[i][j] {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", count);
}
