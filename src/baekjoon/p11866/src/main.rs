fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_josephus_vector(n: usize, k: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = (0 + 1..=n).collect();
    let mut result = Vec::new();
    let mut index = 0;

    loop {
        let len = vec.len();
        index = (index + k - 1) % len;
        result.push(vec[index]);
        vec.remove(index);
        if vec.len() == 0 {
            break;
        }
    }
    result
}

fn print_vec(vec: Vec<usize>) {
    let results = vec
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>();
    let result = results.join(", ");
    
    println!("<{}>", result);
}

fn main() {
    let nums = read_line_as_numbers();
    let (n, k) = (nums[0], nums[1]);
    let result = get_josephus_vector(n, k);
    print_vec(result);
}
