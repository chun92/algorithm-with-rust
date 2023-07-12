fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_count(map: &Vec<Vec<usize>>, n: usize, h: usize) -> usize {let map = map
    .iter()
    .map(|row| {
        row.iter()
            .map(|&x| if x < h { false } else { true })
            .collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();
    let mut count = 0;
    let mut visitied = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if visitied[i][j] {
                continue;
            }

            if !map[i][j] {
                continue;
            }

            let mut stack = Vec::new();
            stack.push((i, j));

            while let Some((i, j)) = stack.pop() {
                visitied[i][j] = true;
                if i > 0 && !visitied[i - 1][j] && map[i - 1][j] {
                    stack.push((i - 1, j));
                }

                if i < n - 1 && !visitied[i + 1][j] && map[i + 1][j] {
                    stack.push((i + 1, j));
                }

                if j > 0 && !visitied[i][j - 1] && map[i][j - 1] {
                    stack.push((i, j - 1));
                }

                if j < n - 1 && !visitied[i][j + 1] && map[i][j + 1] {
                    stack.push((i, j + 1));
                }
            }
            count += 1;
        }
    }
    count
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut map = Vec::new();

    for _ in 0..n {
        map.push(read_line_as_numbers());
    }

    let mut max = 0;

    for i in 0..101 {
        let count = get_count(&map, n, i);
        if count == 0 {
            break;
        }
        max = std::cmp::max(max, count);
    }
    println!("{}", max);
}
