use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut iter = numbers.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let third = iter.next().unwrap();

    let result = match (*first, *second, *third) {
        (x, y, z) if x == y && y == z => 10000 + x * 1000,
        (x, y, _) if x == y => 1000 + x * 100,
        (_, y, z) if y == z => 1000 + y * 100,
        (x, _, z) if z == x => 1000 + z * 100,
        (x, y, z) => {
            let v = vec![x, y, z];
            *v.iter().max().unwrap() * 100
        }
    };

    println!("{}", result);
}
