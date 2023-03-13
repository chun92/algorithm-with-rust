fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn build_vector(n: usize, _m: usize) -> Vec<Vec<i64>> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read_line_as_numbers());
    }
    v
}

fn is_possible_height(v: &Vec<Vec<i64>>, b: i64, height: i64) -> bool {
    let mut count = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            count += height - v[i][j];
        }
    }

    count <= b
}

fn get_time(v: &Vec<Vec<i64>>, height: i64) -> u64 {
    let mut time = 0;
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            let n = height - v[i][j];

            time += if n > 0 { n as u64 } else { (-n * 2) as u64 };
        }
    }

    time
}

fn solve(v: &Vec<Vec<i64>>, b: i64) -> (u64, i64) {
    let mut cur_height = 0;
    let mut cur_time = u64::MAX;

    for i in 0..=256 {
        if !is_possible_height(v, b, i) {
            break;
        }

        let time = get_time(v, i);
        if time <= cur_time {
            cur_time = time;
            cur_height = i;
        }
    }
    
    (cur_time, cur_height)
}

fn main() {
    let args = read_line_as_numbers();
    let (n, m, b) = (args[0] as usize, args[1] as usize, args[2]);
    let vec = build_vector(n, m);
    let result = solve(&vec, b);
    println!("{} {}", result.0, result.1);
}
