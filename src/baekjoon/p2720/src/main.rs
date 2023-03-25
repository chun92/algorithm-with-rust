fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_coins(target: i32) -> (i32, i32, i32, i32) {
    let mut target = target;
    let coin_25 = target / 25;
    target -= coin_25 * 25;
    let coin_10 = target / 10;
    target -= coin_10 * 10;
    let coin_5 = target / 5;
    target -= coin_5 * 5;
    let coin_1 = target;
    (coin_25, coin_10, coin_5, coin_1)
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        let target = read_line_as_number();
        let (coin_25, coin_10, coin_5, coin_1) = get_coins(target);
        println!("{} {} {} {}", coin_25, coin_10, coin_5, coin_1);
    }
}
