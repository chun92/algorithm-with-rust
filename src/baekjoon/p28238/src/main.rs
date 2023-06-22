fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut count = vec![0; 1 << 5];
    let check = vec![3, 5, 6, 9, 10, 12, 17, 18, 20, 24];
    for _ in 0..n {
        let line = read_line_as_numbers();
        let (a, b, c, d, e) = (line[0], line[1], line[2], line[3], line[4]);
        let target = a << 4 | b << 3 | c << 2 | d << 1 | e;

        for j in &check {
            if target & *j == *j {
                count[*j] += 1;
            }
        }
    }
    
    let mut max = 0;
    let mut max_index = 3;
    
    for j in &check {
        if count[*j] > max {
            max = count[*j];
            max_index = *j;
        }
    }

    println!("{}", max);
    for i in (0..5).rev() {
        if max_index & 1 << i == 1 << i {
            print!("{} ", 1);
        } else {
            print!("{} ", 0);
        }
    }
}
