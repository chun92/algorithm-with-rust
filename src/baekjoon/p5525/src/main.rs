use std::collections::HashMap;

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_line_and_count_ioi() -> HashMap<usize, usize> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let result = input
        .chars()
        .fold((HashMap::new(), 0, ' ', false), |acc, c| {
            match (acc.3, acc.2, c) {
                (false, _, 'I') => (acc.0, acc.1, c, true),
                (false, _, _) => (acc.0, acc.1, c, false),
                (true, 'I', 'O') => (acc.0, acc.1, c, true),
                (true, 'O', 'I') => (acc.0, acc.1 + 1, c, true),
                (true, _, 'I') => {
                    let mut map = acc.0;
                    if acc.1 > 0 {
                        *map.entry(acc.1).or_insert(0) += 1;
                    }
                    (map, 0, c, true)
                },
                (true, _, _) => {
                    let mut map = acc.0;
                    if acc.1 > 0 {
                        *map.entry(acc.1).or_insert(0) += 1;
                    }
                    (map, 0, c, false)
                }
            }
        });
    if result.3 {
        let mut map = result.0;
        *map.entry(result.1).or_insert(0) += 1;
        map
    } else {
        result.0
    }
}

fn main() {
    let n = read_line_as_number();
    let _length = read_line_as_number();
    let map = read_line_and_count_ioi();
    let result = map
        .iter()
        .fold(0, |acc, (k, v)| if k >= &n { acc + v * (k - n + 1) } else { acc });
    println!("{}", result);
}
