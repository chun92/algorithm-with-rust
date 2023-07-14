fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let x = read_line_as_number();
    let mut vec = vec![64];

    loop {
        let sum = vec.iter().sum::<usize>();
        if x == sum {
            break;
        }
        vec.sort();
        let min = vec[0];
        let half = min / 2;
        if sum - half >= x {
            vec[0] = half;
        } else {
            vec[0] = half;
            vec.push(half);
        }
    }

    println!("{}", vec.len());
}
