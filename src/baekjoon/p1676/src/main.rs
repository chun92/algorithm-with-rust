fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_one(fact_2: i32, fact_5: i32, n: i32) -> (i32, i32) {
    let mut cur = n;
    let mut my_fact_2 = 0;
    let mut my_fact_5 = 0;
    while cur > 0 {
        if cur % 2 == 0 {
            my_fact_2 += 1;
            cur /= 2;
        } if cur % 5 == 0 {
            my_fact_5 += 1;
            cur /= 5;
        } else {
            break;
        }
    }

    (my_fact_2 + fact_2, my_fact_5 + fact_5)
}

fn print_result(n: i32) {
    let mut result = (0, 0);
    for i in 1..=n {
        result = get_one(result.0, result.1, i);
    }

    println!("{}", result.0.min(result.1));
}

fn main() {
    let n = read_line_as_number();
    print_result(n);
}
