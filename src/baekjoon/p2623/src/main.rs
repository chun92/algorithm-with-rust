fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut enters = vec![0; n + 1];
    let mut graphs = vec![vec![]; n + 1];

    for _ in 0..m {
        let vec = read_line_as_numbers();
        let len = vec[0];
        for i in 1..len {
            graphs[vec[i]].push(vec[i + 1]);
            enters[vec[i + 1]] += 1;
        }
    }

    let mut vec = Vec::new();

    for i in 1..=n {
        if enters[i] == 0 {
            vec.push(i);
        }
    }

    let mut result = Vec::new();
    
    while let Some(node) = vec.pop() {
        result.push(node);
        for &next in &graphs[node] {
            enters[next] -= 1;
            if enters[next] == 0 {
                vec.push(next);
            }
        }
    }

    for i in 1..=n {
        if enters[i] != 0 {
            println!("0");
            return;
        }
    }

    for i in result {
        println!("{}", i);
    }

}
