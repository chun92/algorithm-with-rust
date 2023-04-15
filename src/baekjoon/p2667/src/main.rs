fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn main() {
    let n = read_line_as_number();
    let mut numbers = Vec::new();
    let mut visitied = vec![vec![false; n]; n];
    for _ in 0..n {
        let line = read_line_as_numbers();
        numbers.push(line);
    }

    let mut result = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if visitied[i][j] {
                continue;
            }
            let mut count = 0;
            let mut queue = Vec::new();
            queue.push((i, j));
            while !queue.is_empty() {
                let (x, y) = queue.pop().unwrap();
                if numbers[x][y] == 0 {
                    continue;
                }
                if visitied[x][y] {
                    continue;
                }
                visitied[x][y] = true;
                count += 1;
                if x > 0 && numbers[x - 1][y] == 1 {
                    queue.push((x - 1, y));
                }
                if x < n - 1 && numbers[x + 1][y] == 1 {
                    queue.push((x + 1, y));
                }
                if y > 0 && numbers[x][y - 1] == 1 {
                    queue.push((x, y - 1));
                }
                if y < n - 1 && numbers[x][y + 1] == 1 {
                    queue.push((x, y + 1));
                }
            }

            if count != 0 {
                result.push(count);
            }
        }
    }

    result.sort();
    println!("{}", result.len());
    result
        .iter()
        .for_each(|x| println!("{}", x));
}
