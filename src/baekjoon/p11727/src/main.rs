fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();

    if n == 1 {
        println!("1");
        return;
    }

    if n == 2 {
        println!("3");
        return;
    }

    let mut vec = Vec::new();

    vec.push(1);
    vec.push(3);

    for _ in 2..n {
        let last = vec.last().unwrap();
        let second_last = vec[vec.len() - 2];

        let num = (last + second_last * 2) % 10007;
        vec.push(num);
    }

    println!("{}", vec.last().unwrap());
}
