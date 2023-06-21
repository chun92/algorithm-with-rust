fn read_line_as_numbers() -> Vec<u8> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn solve(vec: &Vec<(usize, usize)>, x: usize, y: usize, index: usize, depth: usize, max: &mut usize) {
    if index == vec.len() {
        if depth > *max {
            *max = depth;
        }
        return;
    }

    let mut index = index;
    loop {
        if index == vec.len() {
            break;
        }
        let (x1, y1) = vec[index];
        if 1 << x1 & x == 0 && 1 << y1 & y == 0 {
            solve(vec, 1 << x1 | x, 1 << y1 | y, index + 1, depth + 1, max);
        }
        index += 1;
    }
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;

    let mut black = Vec::new();
    let mut white = Vec::new();
    for i in 0..n {
        let line = read_line_as_numbers();
        for j in 0..n {
            if line[j] == 0 {
                continue;
            }
            let x = n - 1 + i - j;
            let y = i + j;
            if y % 2 == 0 {
                black.push((x, y));
            } else {
                white.push((x, y));
            }
        }
    }

    let mut max1 = 0;
    solve(&black, 0, 0, 0, 0, &mut max1);
    let mut max2 = 0;
    solve(&white, 0, 0, 0, 0, &mut max2);
    println!("{}", max1 + max2);
}
