fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_1s_as_binary(target: usize) -> usize {
    let mut result = 0;
    let mut n = target;
    while n > 0 {
        if n % 2 == 1 {
            result += 1;
        }
        n = n >> 1;
    }
    result
}

fn get_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn main() {
    let (n, m) = {
        let line = read_line_as_numbers();
        (line[0], line[1])
    };

    let mut houses = Vec::new();
    let mut chickens = Vec::new();

    for i in 0..n {
        let line = read_line_as_numbers();
        line
            .iter()
            .enumerate()
            .for_each(|(j, &v)| match v {
                1 => houses.push((i, j)),
                2 => chickens.push((i, j)),
                _ => {}
            });
    }
    let num_of_chickens = chickens.len();

    let mut result = usize::MAX;
    for i in 0..1 << num_of_chickens {
        if get_1s_as_binary(i) != m {
            continue;
        }

        let mut distances = vec![usize::MAX; houses.len()];
        for j in 0..num_of_chickens {
            if i & (1 << j) == 0 {
                continue;
            }
            let point = chickens[j];
            for (k, &house) in houses.iter().enumerate() {
                let distance = get_distance(point, house);
                if distance < distances[k] {
                    distances[k] = distance;
                }
            }
            let sum: usize = distances.iter().sum();
            if sum < result {
                result = sum;
            }
        }
    }
    println!("{}", result);
}
