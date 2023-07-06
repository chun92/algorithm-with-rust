fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut vec = read_line_as_numbers();
    let mut s = read_line_as_numbers()[0];

    let mut current_target_index = 0;
    while s > 0 {
        if current_target_index >= n {
            break;
        }

        let mut target_vec = vec[current_target_index..].to_vec().iter().enumerate().map(|(i, x)| (i, *x)).collect::<Vec<(usize, usize)>>();
        target_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let target_index;
        let mut i = 0;
        loop {
            let (index, _) = target_vec[i];
            if index <= s {
                target_index = index;
                break;
            } else {
                i += 1;
            }
        }

        for i in (current_target_index + 1..current_target_index + target_index + 1).rev() {
            let temp = vec[i];
            vec[i] = vec[i - 1];
            vec[i - 1] = temp;
        }

        s -= target_index;
        current_target_index += 1;
    }

    for i in 0..n {
        print!("{} ", vec[i]);
    }
}
