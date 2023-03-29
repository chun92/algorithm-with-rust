use std::collections::HashMap;

fn read_line_as_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn get_nth_num(n: u32, hash_map: &mut HashMap<u32, u32>) -> u32 {
    if hash_map.contains_key(&n) {
        return hash_map[&n];
    }

    if n == 1 {
        hash_map.insert(1, 1);
        return 1;
    
    }
    if n == 2 {
        hash_map.insert(2, 2);
        return 2;
    
    }
    if n == 3 {
        hash_map.insert(3, 4);
        return 4;
    }

    let result = get_nth_num(n - 1, hash_map) + get_nth_num(n - 2, hash_map) + get_nth_num(n - 3, hash_map);
    hash_map.insert(n, result);
    result
}

fn main() {
    let n = read_line_as_number();

    let mut hash_map = HashMap::new();

    for _ in 0..n {
        let num = read_line_as_number();
        let result = get_nth_num(num, &mut hash_map);
        println!("{}", result);
    }
}
