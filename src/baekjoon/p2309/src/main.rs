fn read_line_as_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn next_combination(vec: &mut Vec<usize>, n: usize) -> bool {
    let mut i = vec.len() - 1;
    while i > 0 && vec[i] == n - vec.len() + i {
        i -= 1;
    }
    if i == 0 && vec[i] == n - vec.len() {
        return false;
    }
    vec[i] += 1;
    for j in i..vec.len() {
        vec[j] = vec[i] + j - i;
    }
    true
}

fn main() {
    let mut vec = Vec::new();
    for _ in 0..9 {
        let number = read_line_as_number();
        vec.push(number);
    }

    let mut idx = (0..7).collect::<Vec<_>>();
    
    loop {
        let mut sum = 0;
        for i in &idx {
            sum += vec[*i];
        }
        if sum == 100 {
            break;
        }
        if !next_combination(&mut idx, 9) {
            break;
        }
    }

    let mut result = Vec::new();
    for i in &idx {
        result.push(vec[*i]);
    }
    result.sort();
    result
        .iter()
        .for_each(|x| println!("{}", x));
}