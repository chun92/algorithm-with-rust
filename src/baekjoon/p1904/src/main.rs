use std::collections::HashMap;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn solve(n: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 2;
    }

    map.insert(1, 1);
    map.insert(2, 2);

    for i in 3..=n {
        let res = (map[&(i - 1)] + map[&(i - 2)]) % 15746;
        map.insert(i, res);
    }
    map[&n]
}

fn main() {
    let n = read_line_as_number();
    println!("{}", solve(n));
}
