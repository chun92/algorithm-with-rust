use std::collections::HashMap;

fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn get_wave(n: u64) -> u64 {
    let mut map = HashMap::new();

    map.insert(1, 1);
    map.insert(2, 1);
    map.insert(3, 1);
    map.insert(4, 2);
    map.insert(5, 2);

    for i in 6..=n {
        map.insert(i, map.get(&(i - 1)).unwrap() + map.get(&(i - 5)).unwrap());
    }

    map[&n]
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        println!("{}", get_wave(read_line_as_number()));
    }
}
