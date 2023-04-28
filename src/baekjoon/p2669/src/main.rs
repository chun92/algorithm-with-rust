fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut graph = vec![vec![false; 100]; 100];

    for _ in 0..4 {
        let numbers = read_line_as_numbers();
        let x1 = numbers[0];
        let y1 = numbers[1];
        let x2 = numbers[2];
        let y2 = numbers[3];
        for i in x1..x2 {
            for j in y1..y2 {
                graph[i][j] = true;
            }
        }
    }

    let mut answer = 0;
    for i in 0..100 {
        for j in 0..100 {
            if graph[i][j] {
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
