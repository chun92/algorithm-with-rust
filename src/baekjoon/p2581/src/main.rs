fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn set_primes(prime_vec: &mut Vec<bool>) {
    prime_vec[0] = false;
    prime_vec[1] = false;
    let mut i = 2;
    while i < prime_vec.len() {
        let mut j = i * 2;
        if prime_vec[i] {
            while j < prime_vec.len() {
                prime_vec[j] = false;
                j += i;
            }
        }
        i += 1;
    }
}

fn main() {
    let mut prime_vec = vec![true;10001];
    set_primes(&mut prime_vec);

    let n = read_line_as_number();
    let m = read_line_as_number();

    let mut results_vec: Vec<i32> = Vec::new();
    for i in n..=m {
        if prime_vec[i] {
            results_vec.push(i as i32);
        }
    }

    let sum: i32 = results_vec.iter().sum();
    let min = results_vec.iter().min().unwrap_or(&-1);
    if sum != 0 {
        println!("{}", sum);
    }
    println!("{}", min);
}
