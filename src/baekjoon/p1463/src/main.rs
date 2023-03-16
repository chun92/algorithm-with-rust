fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn dp(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }

    if n == 2 || n == 3 {
        return 1;
    }
    
    let mut vec = vec![0; 1000001];
    vec[1] = 1;
    vec[2] = 1;
    vec[3] = 1;

    for i in 4..=n {
        let mut candidate = Vec::new();
        if i % 3 == 0 {
            candidate.push(vec[(i / 3) as usize]);
        }
        if i % 2 == 0 {
            candidate.push(vec[(i / 2) as usize]);
        }
        candidate.push(vec[(i - 1) as usize]);

        let result = candidate.iter().min().unwrap() + 1;
        vec[i as usize] = result;
    }
    vec[n as usize]
}


fn main() {
    let n = read_line_as_number();
    println!("{}", dp(n));
}
