use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let m = read_line_as_numbers()[0];

    let mut map: HashMap<usize, (Vec<usize>, bool)> = HashMap::new();
    (1..=n).for_each(|x| {
        map.insert(x, (Vec::new(), false));
    });

    for _ in 0..m {
        let (a, b) = {
            let nums = read_line_as_numbers();
            (nums[0], nums[1])
        };
        map.get_mut(&a).unwrap().0.push(b);
        map.get_mut(&b).unwrap().0.push(a);
    }

    let mut target_vec = vec![1];
    while !target_vec.is_empty() {
        let target = target_vec.pop().unwrap();
        let (vec, visited) = map.get_mut(&target).unwrap();
        if !*visited {
            *visited = true;
            target_vec.extend(vec.iter());
        }
    }

    let ans = map.values().filter(|(_, visited)| *visited).count() - 1;
    println!("{}", ans);
}
