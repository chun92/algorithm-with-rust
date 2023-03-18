use std::io::{Write, BufWriter, BufRead, StdinLock,stdin, stdout};

fn read_line_as_numbers(stdin_lock: &mut StdinLock) -> Vec<usize> {
    let mut line = String::new();
    stdin_lock.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn fill_graph(n: usize, stdin_lock: &mut StdinLock, graph: &mut Vec<Vec<usize>>) {
    for i in 0..n {
        let nums = read_line_as_numbers(stdin_lock);
        let mut sum = 0;
        let mut input = Vec::new();
        for (j, v) in nums.iter().enumerate() {
            let mut result = 0;
            sum += v;
            if i != 0 {
                result += graph[i - 1][j];
            }
            result += sum;
            input.push(result);
        }
        graph.push(input);
    }
}

fn get_partial_sum(graph: &Vec<Vec<usize>>, x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let mut result = graph[x2][y2];
    if x1 != 0 && y1 != 0 {
        result += graph[x1 - 1][y1 - 1];
    }
    if x1 != 0 {
        result -= graph[x1 - 1][y2];
    }
    if y1 != 0 {
        result -= graph[x2][y1 - 1];
    }
    result
}

fn main() {
    let stdin = stdin();
    let mut stdin_lock = stdin.lock();

    let args = read_line_as_numbers(&mut stdin_lock);
    let (n, m) = (args[0], args[1]);

    let mut graph = Vec::new();
    fill_graph(n, &mut stdin_lock, &mut graph);

    let mut stdout = BufWriter::new(stdout());
    for _ in 0..m {
        let args = read_line_as_numbers(&mut stdin_lock);
        let (x1, y1, x2, y2) = (args[0] - 1, args[1] - 1, args[2] - 1, args[3] - 1);
        writeln!(stdout, "{}", get_partial_sum(&graph, x1, y1, x2, y2)).unwrap();
    }
}
