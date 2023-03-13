use std::collections::HashMap;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn fibonacci(n: i32, map: &mut HashMap<i32, i32>) -> (i32, i32) {
    if n == 1 {
        return (1, 0);
    }
    if n == 2 {
        return (1, 0);
    }
    
    map.insert(1, 1);
    
    map.insert(2, 1);

    for i in 3..=n {
        match map.get(&i) {
            Some(_) => { },
            None => {
                let v = map[&(i - 1)] + map[&(i - 2)] ;
                map.insert(i, v);
            }
        }
    }
    
    return (map[&n], n - 2);
}

fn main() {
    let n = read_line_as_number();
    let mut map = HashMap::new();
    let result = fibonacci(n, &mut map);
    println!("{} {}", result.0, result.1);
}
